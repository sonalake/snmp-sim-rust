/*
   Copy/paste code from async-oneshot crate.
   The goal is to reduce one dependency(futures-micro) for not using wait method of
   async_oneshot::Sender
*/

use core::{
    cell::UnsafeCell,
    future::Future,
    mem::MaybeUninit,
    pin::Pin,
    ptr::drop_in_place,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll, Waker},
};

use crate::error::ActixAsyncError;
use crate::util::smart_pointer::RefCounter;

pub(crate) fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let inner = RefCounter::new(Inner::new());
    let sender = OneshotSender::new(inner.clone());
    let receiver = OneshotReceiver::new(inner);
    (sender, receiver)
}

#[derive(Debug)]
pub(crate) struct Inner<T> {
    state: AtomicUsize,
    value: UnsafeCell<MaybeUninit<T>>,
    send: UnsafeCell<MaybeUninit<Waker>>,
    recv: UnsafeCell<MaybeUninit<Waker>>,
}

const CLOSED: usize = 0b1000;
const SEND: usize = 0b0100;
const RECV: usize = 0b0010;
const READY: usize = 0b0001;

impl<T> Inner<T> {
    pub(crate) const fn new() -> Self {
        Inner {
            state: AtomicUsize::new(0),
            value: UnsafeCell::new(MaybeUninit::uninit()),
            send: UnsafeCell::new(MaybeUninit::uninit()),
            recv: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    // Gets the current state
    pub(crate) fn state(&self) -> State {
        State(self.state.load(Ordering::Acquire))
    }

    // Gets the receiver's waker. You *must* check the state to ensure
    // it is set. This would be unsafe if it were public.
    pub(crate) fn recv(&self) -> &Waker {
        // MUST BE SET
        debug_assert!(self.state().recv());
        unsafe { &*(*self.recv.get()).as_ptr() }
    }

    // Sets the receiver's waker.
    pub(crate) fn set_recv(&self, waker: Waker) -> State {
        let recv = self.recv.get();
        unsafe { (*recv).as_mut_ptr().write(waker) } // !
        State(self.state.fetch_or(RECV, Ordering::AcqRel))
    }

    // Gets the sender's waker. You *must* check the state to ensure
    // it is set. This would be unsafe if it were public.
    pub(crate) fn send(&self) -> &Waker {
        debug_assert!(self.state().send());
        unsafe { &*(*self.send.get()).as_ptr() }
    }

    pub(crate) fn take_value(&self) -> T {
        // MUST BE SET
        debug_assert!(self.state().ready());
        unsafe { (*self.value.get()).as_ptr().read() }
    }

    pub(crate) fn set_value(&self, value: T) -> State {
        debug_assert!(!self.state().ready());
        let val = self.value.get();
        unsafe { (*val).as_mut_ptr().write(value) }
        State(self.state.fetch_or(READY, Ordering::AcqRel))
    }

    pub(crate) fn close(&self) -> State {
        State(self.state.fetch_or(CLOSED, Ordering::AcqRel))
    }
}

impl<T> Drop for Inner<T> {
    fn drop(&mut self) {
        let state = State(*self.state.get_mut());
        // Drop the wakers if they are present
        if state.recv() {
            unsafe {
                drop_in_place((&mut *self.recv.get()).as_mut_ptr());
            }
        }
        if state.send() {
            unsafe {
                drop_in_place((&mut *self.send.get()).as_mut_ptr());
            }
        }
    }
}

unsafe impl<T: Send> Send for Inner<T> {}
unsafe impl<T: Sync> Sync for Inner<T> {}

#[derive(Clone, Copy)]
pub(crate) struct State(usize);

impl State {
    pub(crate) fn closed(&self) -> bool {
        (self.0 & CLOSED) == CLOSED
    }
    pub(crate) fn ready(&self) -> bool {
        (self.0 & READY) == READY
    }
    pub(crate) fn send(&self) -> bool {
        (self.0 & SEND) == SEND
    }
    pub(crate) fn recv(&self) -> bool {
        (self.0 & RECV) == RECV
    }
}

#[derive(Debug, Clone)]
pub struct OneshotSender<T> {
    inner: RefCounter<Inner<T>>,
    done: bool,
}

impl<T> OneshotSender<T> {
    pub(crate) fn new(inner: RefCounter<Inner<T>>) -> Self {
        OneshotSender { inner, done: false }
    }

    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        self.inner.state().closed()
    }

    #[inline]
    pub fn send(mut self, value: T) -> Result<(), ActixAsyncError> {
        self.done = true;
        let inner = &mut self.inner;
        let state = inner.set_value(value);
        if !state.closed() {
            if state.recv() {
                inner.recv().wake_by_ref();
                Ok(())
            } else {
                Ok(())
            }
        } else {
            inner.take_value();
            Err(ActixAsyncError::Closed)
        }
    }
}

impl<T> Drop for OneshotSender<T> {
    fn drop(&mut self) {
        if !self.done {
            let state = self.inner.state();
            if !state.closed() {
                let old = self.inner.close();
                if old.recv() {
                    self.inner.recv().wake_by_ref();
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct OneshotReceiver<T> {
    inner: RefCounter<Inner<T>>,
    done: bool,
}

impl<T> OneshotReceiver<T> {
    pub(crate) fn new(inner: RefCounter<Inner<T>>) -> Self {
        OneshotReceiver { inner, done: false }
    }

    fn handle_state(&mut self, state: State) -> Poll<Result<T, ActixAsyncError>> {
        if state.ready() {
            Poll::Ready(Ok(self.inner.take_value()))
        } else if state.closed() {
            Poll::Ready(Err(ActixAsyncError::Closed))
        } else {
            Poll::Pending
        }
        .map(|x| {
            self.done = true;
            x
        })
    }
}

impl<T> Future for OneshotReceiver<T> {
    type Output = Result<T, ActixAsyncError>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Result<T, ActixAsyncError>> {
        let this = Pin::into_inner(self);
        match this.handle_state(this.inner.state()) {
            Poll::Pending => {}
            x => return x,
        }
        let state = this.inner.set_recv(ctx.waker().clone());
        match this.handle_state(state) {
            Poll::Pending => {}
            x => return x,
        }
        if state.send() {
            this.inner.send().wake_by_ref();
        }
        Poll::Pending
    }
}

impl<T> Drop for OneshotReceiver<T> {
    fn drop(&mut self) {
        if !self.done {
            let state = self.inner.state();
            if !state.closed() && !state.ready() {
                let old = self.inner.close();
                if old.send() {
                    self.inner.send().wake_by_ref();
                }
            }
        }
    }
}
