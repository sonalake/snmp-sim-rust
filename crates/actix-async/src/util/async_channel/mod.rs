/// Copy/paste from async-channel crate.
///
/// The goal is to add weak sender and poll method for sender.
/// The channel has also been modified to have both unbounded and bounded behavior.
mod listener;
mod unbounded;

use core::{
    fmt,
    future::Future,
    pin::Pin,
    sync::atomic::{fence, AtomicUsize, Ordering},
    task::{Context, Poll},
};

use crate::error::ActixAsyncError;
use crate::util::{
    futures::{ready, Stream},
    smart_pointer::{RefCounter, WeakRefCounter},
};

use self::listener::{Event, EventListener};
use self::unbounded::Unbounded;

struct Channel<T> {
    queue: Unbounded<T>,
    in_queue: AtomicUsize,
    cap: usize,
    send_ops: Event,
    stream_ops: Event,
    sender_count: AtomicUsize,
    receiver_count: AtomicUsize,
}

impl<T> Channel<T> {
    fn close(&self) -> bool {
        if self.queue.close() {
            // Notify all send operations.
            self.send_ops.notify(usize::MAX);

            // Notify all stream operations.
            self.stream_ops.notify(usize::MAX);

            true
        } else {
            false
        }
    }

    /// return if there is available count.
    fn dequeue(&self) -> bool {
        self.cap >= self.in_queue.fetch_sub(1, Ordering::Relaxed)
    }
}

pub(crate) fn channel<T>(cap: usize) -> (Sender<T>, Receiver<T>) {
    assert!(cap > 0, "capacity cannot be zero");

    let channel = RefCounter::new(Channel {
        queue: Unbounded::new(),
        cap,
        in_queue: AtomicUsize::new(0),
        send_ops: Event::new(),
        stream_ops: Event::new(),
        sender_count: AtomicUsize::new(1),
        receiver_count: AtomicUsize::new(1),
    });

    let s = Sender {
        channel: channel.clone(),
    };
    let r = Receiver {
        channel,
        listener: None,
    };
    (s, r)
}

pub struct Sender<T> {
    channel: RefCounter<Channel<T>>,
}

impl<T> Sender<T> {
    pub(crate) fn do_send(&self, msg: T) -> Result<(), T> {
        self.channel.queue.push(msg).map(|()| {
            // Notify all blocked streams.
            self.channel.stream_ops.notify(usize::MAX);
        })
    }

    pub(crate) fn send(&self, msg: T) -> SendFuture<'_, T> {
        SendFuture {
            sender: self,
            listener: None,
            msg: Some(msg),
        }
    }

    #[cfg(feature = "std")]
    pub(crate) fn close(&self) -> bool {
        self.channel.close()
    }

    pub(crate) fn downgrade(&self) -> WeakSender<T> {
        WeakSender {
            channel: RefCounter::downgrade(&self.channel),
        }
    }

    fn clone_sender(channel: &RefCounter<Channel<T>>) -> Self {
        let count = channel.sender_count.fetch_add(1, Ordering::Relaxed);

        // Make sure the count never overflows, even if lots of sender clones are leaked.
        if count > usize::MAX / 2 {
            panic!("Sender count overflow");
        }

        Sender {
            channel: RefCounter::clone(channel),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // Decrement the sender count and close the channel if it drops down to zero.
        if self.channel.sender_count.fetch_sub(1, Ordering::AcqRel) == 1 {
            self.channel.close();
        }
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sender {{ .. }}")
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self::clone_sender(&self.channel)
    }
}

pub struct SendFuture<'a, T> {
    sender: &'a Sender<T>,
    listener: Option<EventListener>,
    msg: Option<T>,
}

impl<T> Unpin for SendFuture<'_, T> {}

impl<T> Future for SendFuture<'_, T> {
    type Output = Result<(), ActixAsyncError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        let msg = this.msg.take().unwrap();

        let cap = this.sender.channel.cap;
        let mut in_queue = this.sender.channel.in_queue.load(Ordering::Relaxed);

        loop {
            if in_queue < cap {
                match this.sender.channel.in_queue.compare_exchange_weak(
                    in_queue,
                    in_queue + 1,
                    Ordering::SeqCst,
                    Ordering::Relaxed,
                ) {
                    Ok(cur) => {
                        return match this.sender.do_send(msg) {
                            Ok(_) => {
                                // If the capacity is larger than 1, notify another blocked send operation.
                                match cap - cur {
                                    1 => {}
                                    _ => this.sender.channel.send_ops.notify(1),
                                }
                                Poll::Ready(Ok(()))
                            }
                            // TODO: It's possible to give message's ownership back to caller.
                            Err(_msg) => Poll::Ready(Err(ActixAsyncError::Closed)),
                        };
                    }
                    Err(cur) => {
                        in_queue = cur;
                        // another thread increment the counter already.
                        // (This path should be very short)
                        continue;
                    }
                }
            }

            // Sending failed because channel is full
            // now start listening for notifications or wait for one.
            match this.listener.as_mut() {
                None => {
                    // Start listening and then try receiving again.
                    this.listener = Some(this.sender.channel.send_ops.listen());
                }
                Some(l) => {
                    // Wait for a notification.
                    match Pin::new(l).poll(cx) {
                        Poll::Ready(_) => {
                            this.listener.take();
                            continue;
                        }
                        Poll::Pending => {
                            this.msg = Some(msg);
                            return Poll::Pending;
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct WeakSender<T> {
    channel: WeakRefCounter<Channel<T>>,
}

impl<T> Clone for WeakSender<T> {
    fn clone(&self) -> Self {
        Self {
            channel: WeakRefCounter::clone(&self.channel),
        }
    }
}

impl<T> WeakSender<T> {
    pub fn upgrade(&self) -> Option<Sender<T>> {
        if let Some(channel) = WeakRefCounter::upgrade(&self.channel) {
            let mut count = channel.sender_count.load(Ordering::Relaxed);

            while count != 0 {
                match channel
                    .sender_count
                    .compare_exchange_weak(count, count + 1, Ordering::SeqCst, Ordering::Relaxed)
                {
                    Ok(_) => return Some(Sender { channel }),
                    Err(cur) => count = cur,
                }
            }
        }

        None
    }
}

pub(crate) struct Receiver<T> {
    channel: RefCounter<Channel<T>>,
    listener: Option<EventListener>,
}

impl<T> Receiver<T> {
    pub(crate) fn try_recv(&self) -> Result<T, TryRecvError> {
        self.channel.queue.pop().map(|msg| {
            if self.channel.dequeue() {
                // Notify a single blocked send operation. If the notified operation then sends a
                // message or gets canceled, it will notify another blocked send operation.
                self.channel.send_ops.notify(1);
            }

            msg
        })
    }

    pub(crate) fn as_sender(&self) -> Option<Sender<T>> {
        if self.channel.queue.is_closed() {
            None
        } else {
            Some(Sender::clone_sender(&self.channel))
        }
    }

    pub(crate) fn close(&self) -> bool {
        self.channel.close()
    }

    #[cfg(feature = "std")]
    pub(crate) fn is_closed(&self) -> bool {
        self.channel.queue.is_closed()
    }
}

impl<T> Stream for Receiver<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // If this stream is listening for events, first wait for a notification.
            if let Some(listener) = self.listener.as_mut() {
                ready!(Pin::new(listener).poll(cx));
                self.listener = None;
            }

            loop {
                // Attempt to receive a message.
                match self.try_recv() {
                    Ok(msg) => {
                        // The stream is not blocked on an event - drop the listener.
                        self.listener = None;
                        return Poll::Ready(Some(msg));
                    }
                    Err(TryRecvError::Closed) => {
                        // The stream is not blocked on an event - drop the listener.
                        self.listener = None;
                        return Poll::Ready(None);
                    }
                    Err(TryRecvError::Empty) => {}
                }

                // Receiving failed - now start listening for notifications or wait for one.
                match self.listener.as_mut() {
                    None => {
                        // Create a listener and try sending the message again.
                        self.listener = Some(self.channel.stream_ops.listen());
                    }
                    Some(_) => {
                        // Go back to the outer loop to poll the listener.
                        break;
                    }
                }
            }
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // Decrement the receiver count and close the channel if it drops down to zero.
        if self.channel.receiver_count.fetch_sub(1, Ordering::AcqRel) == 1 {
            self.channel.close();
        }
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Receiver {{ .. }}")
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Receiver<T> {
        let count = self.channel.receiver_count.fetch_add(1, Ordering::Relaxed);

        // Make sure the count never overflows, even if lots of receiver clones are leaked.
        if count > usize::MAX / 2 {
            panic!("Receiver count overflow");
        }

        Receiver {
            channel: self.channel.clone(),
            listener: None,
        }
    }
}

pub(crate) enum TryRecvError {
    Empty,
    Closed,
}

#[inline]
fn full_fence() {
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        // HACK(stjepang): On x86 architectures there are two different ways of executing
        // a `SeqCst` fence.
        //
        // 1. `atomic::fence(SeqCst)`, which compiles into a `mfence` instruction.
        // 2. `_.compare_exchange(_, _, SeqCst)`, which compiles into a `lock cmpxchg` instruction.
        //
        // Both instructions have the effect of a full barrier, but empirical benchmarks have shown
        // that the second one is sometimes a bit faster.
        //
        // The ideal solution here would be to use inline assembly, but we're instead creating a
        // temporary atomic variable and compare-and-exchanging its value. No sane compiler to
        // x86 platforms is going to optimize this away.
        let a = AtomicUsize::new(0);
        let _ = a.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst);
    } else {
        fence(Ordering::SeqCst);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn weak_sender() {
        let (tx, rx) = channel::<u8>(1);

        let count = |num: usize| assert_eq!(tx.channel.sender_count.load(Ordering::SeqCst), num);

        count(1);

        let weak = tx.downgrade();
        count(1);

        {
            let _strong = weak.upgrade().unwrap();
            count(2);
        }

        count(1);

        let weak = tx.downgrade();
        count(1);
        drop(tx);

        assert!(rx.is_closed());
        assert_eq!(rx.channel.sender_count.load(Ordering::SeqCst), 0);
        assert!(weak.upgrade().is_none());
    }
}
