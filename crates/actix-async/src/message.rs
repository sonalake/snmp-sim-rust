use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context as StdContext, Poll},
    time::Duration,
};

use alloc::boxed::Box;

use super::actor::{Actor, ActorState};
use super::handler::{Handler, MessageHandler};
use super::runtime::RuntimeService;
use super::util::{
    channel::{OneshotReceiver, OneshotSender},
    futures::{ready, LocalBoxStream, Stream},
    smart_pointer::RefCounter,
};

/// trait define types goes through actor's `Addr` to it's `Handler`
///
/// # example:
/// ```rust
/// use actix_async::prelude::*;
///
/// struct MyMsg;
///
/// impl Message for MyMsg {
///     // define what type would be the response for MyMsg.
///     type Result = u32;
/// }
///
/// struct MyMsg2;
/// // a short cut macro would do the same thing as above.
/// message!(MyMsg2, u32);
/// ```
pub trait Message: 'static {
    type Result: Send + 'static;
}

impl<M: Message + ?Sized> Message for RefCounter<M> {
    type Result = M::Result;
}

impl<M: Message + ?Sized> Message for Box<M> {
    type Result = M::Result;
}

pub(crate) struct FunctionMessage<F, R> {
    pub(crate) func: F,
    _res: PhantomData<R>,
}

impl<F: Clone, R> Clone for FunctionMessage<F, R> {
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
            _res: PhantomData,
        }
    }
}

impl<F, R> FunctionMessage<F, R> {
    pub(crate) fn new(func: F) -> Self {
        Self {
            func,
            _res: Default::default(),
        }
    }
}

impl<F, R> Message for FunctionMessage<F, R>
where
    F: 'static,
    R: Send + 'static,
{
    type Result = R;
}

pub(crate) struct FunctionMutMessage<F, R> {
    pub(crate) func: F,
    _res: PhantomData<R>,
}

impl<F: Clone, R> Clone for FunctionMutMessage<F, R> {
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
            _res: PhantomData,
        }
    }
}

impl<F, R> FunctionMutMessage<F, R> {
    pub(crate) fn new(func: F) -> Self {
        Self {
            func,
            _res: Default::default(),
        }
    }
}

impl<F, R> Message for FunctionMutMessage<F, R>
where
    F: 'static,
    R: Send + 'static,
{
    type Result = R;
}

// concrete type for dyn MessageHandler trait object that provide the message and the response
// channel.
pub(crate) struct MessageContainer<M: Message> {
    pub(crate) msg: Option<M>,
    pub(crate) tx: Option<OneshotSender<M::Result>>,
}

impl<M: Message> MessageContainer<M> {
    pub(crate) fn take(&mut self) -> (M, Option<OneshotSender<M::Result>>) {
        (self.msg.take().unwrap(), self.tx.take())
    }
}

/*
    SAFETY:
    Message container is construct from either `Context` or `Addr`.

    *. When it's constructed through `Addr`. The caller must make sure the `Message` type
       passed to it is `Send` bound as the object would possibly sent to another thread.
    *. When it's constructed through `Context`. The container remain on it's thread and never
       move to other threads so it's safe to bound to `Send` regardless.
*/
unsafe impl<M: Message> Send for MessageContainer<M> {}

pub(crate) fn message_send_check<M: Message + Send>() {}

// intern type for cloning a MessageObject.
pub(crate) enum ActorMessageClone<A> {
    Ref(Box<dyn MessageObjectClone<A>>),
    Mut(Box<dyn MessageObjectClone<A>>),
}

// the clone would return ActorMessage directly.
impl<A: Actor> ActorMessageClone<A> {
    pub(crate) fn clone(&self) -> ActorMessage<A> {
        match self {
            Self::Ref(obj) => ActorMessage::Ref(obj.clone_object()),
            Self::Mut(obj) => ActorMessage::Mut(obj.clone_object()),
        }
    }
}

pub(crate) trait MessageObjectClone<A> {
    fn clone_object(&self) -> Box<dyn MessageHandler<A> + Send>;
}

impl<A, M> MessageObjectClone<A> for M
where
    A: Actor + Handler<M>,
    M: Message + Sized + Clone + 'static,
{
    fn clone_object(&self) -> Box<dyn MessageHandler<A> + Send> {
        Box::new(MessageContainer {
            msg: Some(self.clone()),
            tx: None,
        })
    }
}

// message would produced in the future passed to Context<Actor>.
pub(crate) struct FutureMessage<A: Actor> {
    delay: Pin<Box<<A::Runtime as RuntimeService>::Sleep>>,
    handle: Option<OneshotReceiver<()>>,
    msg: Option<ActorMessage<A>>,
}

impl<A: Actor> FutureMessage<A> {
    pub(crate) fn new(dur: Duration, rx: OneshotReceiver<()>, msg: ActorMessage<A>) -> Self {
        Self {
            delay: Box::pin(<A::Runtime as RuntimeService>::sleep(dur)),
            handle: Some(rx),
            msg: Some(msg),
        }
    }
}

impl<A: Actor> Future for FutureMessage<A> {
    type Output = Option<ActorMessage<A>>;

    fn poll(self: Pin<&mut Self>, cx: &mut StdContext<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Some(h) = this.handle.as_mut() {
            match Pin::new(h).poll(cx) {
                // handle canceled. resolve with nothing.
                Poll::Ready(Ok(())) => return Poll::Ready(None),
                // handle dropped. the task is now detached.
                Poll::Ready(Err(_)) => this.handle = None,
                Poll::Pending => {}
            }
        }

        this.delay.as_mut().poll(cx).map(|_| this.msg.take())
    }
}

// interval message passed to Context<Actor>.
pub(crate) struct IntervalMessage<A: Actor> {
    dur: Duration,
    delay: Pin<Box<<A::Runtime as RuntimeService>::Sleep>>,
    handle: Option<OneshotReceiver<()>>,
    msg: ActorMessageClone<A>,
}

impl<A: Actor> IntervalMessage<A> {
    pub(crate) fn new(dur: Duration, rx: OneshotReceiver<()>, msg: ActorMessageClone<A>) -> Self {
        Self {
            dur,
            delay: Box::pin(<A::Runtime as RuntimeService>::sleep(dur)),
            handle: Some(rx),
            msg,
        }
    }
}

impl<A: Actor> Stream for IntervalMessage<A> {
    type Item = ActorMessage<A>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut StdContext<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if let Some(h) = this.handle.as_mut() {
            match Pin::new(h).poll(cx) {
                // handle canceled. resolve with nothing.
                Poll::Ready(Ok(())) => return Poll::Ready(None),
                // handle dropped. the task is now detached.
                Poll::Ready(Err(_)) => this.handle = None,
                Poll::Pending => {}
            }
        }

        ready!(Pin::new(&mut this.delay).poll(cx));

        this.delay = Box::pin(<A::Runtime as RuntimeService>::sleep(this.dur));
        // wake self one more time to register the new sleep.
        cx.waker().wake_by_ref();
        Poll::Ready(Some(this.msg.clone()))
    }
}

pub(crate) enum StreamMessage<A: Actor> {
    Interval(IntervalMessage<A>),
    Boxed(LocalBoxStream<'static, ActorMessage<A>>),
}

impl<A: Actor> StreamMessage<A> {
    pub(crate) fn new_interval(msg: IntervalMessage<A>) -> Self {
        Self::Interval(msg)
    }

    pub(crate) fn new_boxed<S>(stream: S) -> Self
    where
        S: Stream<Item = ActorMessage<A>> + 'static,
    {
        Self::Boxed(Box::pin(stream))
    }
}

impl<A: Actor> Stream for StreamMessage<A> {
    type Item = ActorMessage<A>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut StdContext<'_>) -> Poll<Option<Self::Item>> {
        match self.get_mut() {
            StreamMessage::Interval(stream) => Pin::new(stream).poll_next(cx),
            StreamMessage::Boxed(stream) => stream.as_mut().poll_next(cx),
        }
    }
}

pin_project_lite::pin_project! {
    pub(crate) struct StreamContainer<A, S, F> {
        #[pin]
        stream: S,
        handle: Option<OneshotReceiver<()>>,
        to_msg: F,
        _act: PhantomData<A>,
    }
}

impl<A, S, F> StreamContainer<A, S, F> {
    pub(crate) fn new(stream: S, handle: OneshotReceiver<()>, to_msg: F) -> Self {
        Self {
            stream,
            handle: Some(handle),
            to_msg,
            _act: PhantomData,
        }
    }
}

impl<A, S, F> Stream for StreamContainer<A, S, F>
where
    A: Actor + Handler<S::Item>,
    S: Stream,
    S::Item: Message,
    F: FnOnce(S::Item) -> ActorMessage<A> + Copy,
{
    type Item = ActorMessage<A>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut StdContext<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if let Some(h) = this.handle.as_mut() {
            match Pin::new(h).poll(cx) {
                // handle canceled. resolve with nothing.
                Poll::Ready(Ok(())) => return Poll::Ready(None),
                // handle dropped. the task is now detached.
                Poll::Ready(Err(_)) => *this.handle = None,
                Poll::Pending => {}
            }
        }

        match ready!(this.stream.poll_next(cx)) {
            Some(item) => {
                let msg = (this.to_msg)(item);
                Poll::Ready(Some(msg))
            }
            None => Poll::Ready(None),
        }
    }
}

// main type of message goes through actor's channel.
pub enum ActorMessage<A> {
    Ref(Box<dyn MessageHandler<A> + Send>),
    Mut(Box<dyn MessageHandler<A> + Send>),
    State(ActorState, OneshotSender<()>),
}

impl<A> ActorMessage<A> {
    pub(crate) fn new_ref<M>(msg: M, tx: Option<OneshotSender<M::Result>>) -> Self
    where
        A: Handler<M>,
        M: Message,
    {
        Self::Ref(Box::new(MessageContainer { msg: Some(msg), tx }))
    }

    pub(crate) fn new_mut<M>(msg: M, tx: Option<OneshotSender<M::Result>>) -> Self
    where
        A: Handler<M>,
        M: Message,
    {
        Self::Mut(Box::new(MessageContainer { msg: Some(msg), tx }))
    }
}
