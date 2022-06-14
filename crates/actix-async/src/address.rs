use core::ops::Deref;

use alloc::boxed::Box;

use super::actor::{Actor, ActorState};
use super::context::Context;
use super::error::ActixAsyncError;
use super::handler::Handler;
use super::message::{message_send_check, ActorMessage, FunctionMessage, FunctionMutMessage, Message};
use super::request::{BoxedMessageRequest, MessageRequest, _MessageRequest};
use super::runtime::RuntimeService;
use super::util::{
    channel::{oneshot, OneshotSender, Receiver, Sender, WeakSender},
    futures::LocalBoxFuture,
};

/// The message sink of `Actor` type. `Message` and boxed async blocks are sent to Actor through it.
#[derive(Debug)]
pub struct Addr<A>(Sender<ActorMessage<A>>);

impl<A> Clone for Addr<A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A> Deref for Addr<A> {
    type Target = Sender<ActorMessage<A>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A: Actor> Addr<A> {
    /// send a concurrent message to actor. `Handler::handle` will be called for concurrent message
    /// processing.
    #[inline]
    pub fn send<M>(&self, msg: M) -> MessageRequest<A, M::Result>
    where
        M: Message + Send,
        A: Handler<M>,
    {
        self._send(|tx| ActorMessage::new_ref(msg, Some(tx)))
    }

    /// send an exclusive message to actor. `Handler::handle_wait` will be called for exclusive
    /// message processing.
    /// If `Handler::handle_wait` is not override then it would use `Handler::handle` as fallback.
    #[inline]
    pub fn wait<M>(&self, msg: M) -> MessageRequest<A, M::Result>
    where
        M: Message + Send,
        A: Handler<M>,
    {
        self._send(|tx| ActorMessage::new_mut(msg, Some(tx)))
    }

    /// send a concurrent closure to actor. `Handler::handle` will be called for concurrent message
    /// processing.
    /// closure must be `Send` bound.
    #[inline]
    pub fn run<F, R>(&self, func: F) -> MessageRequest<A, R>
    where
        F: for<'a> FnOnce(&'a A, Context<'a, A>) -> LocalBoxFuture<'a, R> + Send + 'static,
        R: Send + 'static,
    {
        self.send(FunctionMessage::new(func))
    }

    /// send a exclusive closure to actor. `Handler::handle_wait` will be called for exclusive
    /// message processing.
    /// If `Handler::handle_wait` is not override then it would use `Handler::handle` as fallback.
    #[inline]
    pub fn run_wait<F, R>(&self, func: F) -> MessageRequest<A, R>
    where
        F: for<'a> FnOnce(&'a mut A, Context<'a, A>) -> LocalBoxFuture<'a, R> + Send + 'static,
        R: Send + 'static,
    {
        self.wait(FunctionMutMessage::new(func))
    }

    /// send a message to actor and ignore the result.
    ///
    /// This is a synchronous operation that would always queue to actor's mailbox.
    ///
    /// It would bypass `Actor::size_hint` api and would ignore error when doing so.
    /// (Error only happen when actor is closed.)
    ///
    /// *. No guarantee timing of message processing. Actor would use it's size_hint to limit
    /// concurrent async work it runs. All additional messages would remain in mailbox until
    /// it has free slot for processing.
    #[inline]
    pub fn do_send<M>(&self, msg: M)
    where
        M: Message + Send,
        A: Handler<M>,
    {
        self._do_send(|| ActorMessage::new_ref(msg, None))
    }

    /// send an exclusive message to actor and ignore the result.
    ///
    /// This is a synchronous operation that would always queue to actor's mailbox.
    ///
    /// It would bypass `Actor::size_hint` api and would ignore error when doing so.
    /// (Error only happen when actor is closed.)
    ///
    /// *. No guarantee timing of message processing. Actor can only run one exclusive message at
    /// once. All additional messages would remain in mailbox until it has free slot for processing.
    #[inline]
    pub fn do_wait<M>(&self, msg: M)
    where
        M: Message + Send,
        A: Handler<M>,
    {
        self._do_send(|| ActorMessage::new_mut(msg, None))
    }

    /// stop actor.
    ///
    /// When graceful is true the actor would shut it's channel and drain all remaining messages
    /// and exit. When false the actor would exit as soon as the stop message is handled.
    pub fn stop(&self, graceful: bool) -> MessageRequest<A, ()> {
        let state = if graceful {
            ActorState::StopGraceful
        } else {
            ActorState::Stop
        };

        let (tx, rx) = oneshot();

        _MessageRequest::new(self.deref().send(ActorMessage::State(state, tx)), rx)
    }

    /// Weak version of Addr that can be upgraded.
    ///
    /// The upgrade would fail if no `Addr` is alive anywhere.
    #[inline]
    pub fn downgrade(&self) -> WeakAddr<A> {
        WeakAddr(Sender::downgrade(&self.0))
    }

    /// Recipient bound to message type and not actor.
    #[inline]
    pub fn recipient<M>(&self) -> Recipient<A::Runtime, M>
    where
        M: Message + Send,
        A: Handler<M>,
    {
        Recipient(Box::new(self.clone()))
    }

    /// weak version of `Recipient`.
    ///
    /// *. `RecipientWeak` will stay usable as long as the actor and it's `Addr` are alive.
    /// It DOES NOT care if a strong Recipient is alive or not.
    #[inline]
    pub fn recipient_weak<M>(&self) -> RecipientWeak<A::Runtime, M>
    where
        M: Message + Send,
        A: Handler<M>,
    {
        RecipientWeak(Box::new(self.downgrade()))
    }

    pub(crate) fn new(tx: Sender<ActorMessage<A>>) -> Self {
        Self(tx)
    }

    pub(crate) fn from_recv(rx: &Receiver<ActorMessage<A>>) -> Result<Self, ActixAsyncError> {
        match rx.as_sender() {
            Some(tx) => Ok(Addr::new(tx)),
            None => Err(ActixAsyncError::Closed),
        }
    }

    fn _send<M, F>(&self, f: F) -> MessageRequest<A, M::Result>
    where
        A: Handler<M>,
        M: Message + Send,
        F: FnOnce(OneshotSender<M::Result>) -> ActorMessage<A>,
    {
        send(f, |msg| self.deref().send(msg))
    }

    fn _send_box<M, F>(&self, f: F) -> BoxedMessageRequest<A::Runtime, M::Result>
    where
        A: Handler<M>,
        M: Message + Send,
        F: FnOnce(OneshotSender<M::Result>) -> ActorMessage<A>,
    {
        send(f, |msg| Box::pin(self.deref().send(msg)) as _)
    }

    fn _do_send<M, F>(&self, f: F)
    where
        A: Handler<M>,
        M: Message + Send,
        F: FnOnce() -> ActorMessage<A> + 'static,
    {
        message_send_check::<M>();
        let _ = self.deref().do_send(f());
    }
}

fn send<A, M, F, FS, Fut>(f: F, fs: FS) -> _MessageRequest<A::Runtime, Fut, M::Result>
where
    A: Actor + Handler<M>,
    M: Message + Send,
    F: FnOnce(OneshotSender<M::Result>) -> ActorMessage<A>,
    FS: FnOnce(ActorMessage<A>) -> Fut,
{
    message_send_check::<M>();
    let (tx, rx) = oneshot();
    let msg = f(tx);
    _MessageRequest::new(fs(msg), rx)
}

/// weak version `Addr`. Can upgrade to `Addr` when at least one instance of `Addr` is still in
/// scope.
pub struct WeakAddr<A>(WeakSender<ActorMessage<A>>);

impl<A> Clone for WeakAddr<A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: Actor> WeakAddr<A> {
    /// Try to upgrade to a `Addr`
    ///
    /// The upgrade would fail if no `Addr` is alive anywhere.
    #[inline]
    pub fn upgrade(&self) -> Option<Addr<A>> {
        self.0.upgrade().map(Addr)
    }

    fn send_weak<M, F>(&self, f: F) -> BoxedMessageRequest<A::Runtime, M::Result>
    where
        A: Handler<M>,
        M: Message + Send,
        F: FnOnce(OneshotSender<M::Result>) -> ActorMessage<A>,
    {
        send(f, |msg| Box::pin(self._send_weak(msg)) as _)
    }

    async fn _send_weak(&self, msg: ActorMessage<A>) -> Result<(), ActixAsyncError> {
        self.upgrade()
            .ok_or(ActixAsyncError::Closed)?
            .deref()
            .send(msg)
            .await
    }
}

/// trait to bind a given `Addr<A>` or `WeakAddr<A>` to `Message` trait type.
pub trait AddrHandler<RT, M>
where
    RT: RuntimeService,
    M: Message + Send,
    Self: Send + Sync + 'static,
{
    fn send(&self, msg: M) -> BoxedMessageRequest<RT, M::Result>;

    fn wait(&self, msg: M) -> BoxedMessageRequest<RT, M::Result>;

    fn do_send(&self, msg: M);

    fn do_wait(&self, msg: M);
}

impl<A, M> AddrHandler<A::Runtime, M> for Addr<A>
where
    A: Actor + Handler<M>,
    M: Message + Send,
{
    #[inline]
    fn send(&self, msg: M) -> BoxedMessageRequest<A::Runtime, M::Result> {
        self._send_box(|tx| ActorMessage::new_ref(msg, Some(tx)))
    }

    #[inline]
    fn wait(&self, msg: M) -> BoxedMessageRequest<A::Runtime, M::Result> {
        self._send_box(|tx| ActorMessage::new_mut(msg, Some(tx)))
    }

    #[inline]
    fn do_send(&self, msg: M) {
        Addr::do_send(self, msg);
    }

    #[inline]
    fn do_wait(&self, msg: M) {
        Addr::do_wait(self, msg);
    }
}

impl<A, M> AddrHandler<A::Runtime, M> for WeakAddr<A>
where
    A: Actor + Handler<M>,
    M: Message + Send,
{
    #[inline]
    fn send(&self, msg: M) -> BoxedMessageRequest<A::Runtime, M::Result> {
        self.send_weak(|tx| ActorMessage::new_ref(msg, Some(tx)))
    }

    #[inline]
    fn wait(&self, msg: M) -> BoxedMessageRequest<A::Runtime, M::Result> {
        self.send_weak(|tx| ActorMessage::new_mut(msg, Some(tx)))
    }

    /// `AddrHandler::do_send` will panic if the `Addr` for `RecipientWeak` is gone.
    #[inline]
    fn do_send(&self, msg: M) {
        let addr = &self.upgrade().unwrap();
        Addr::do_send(addr, msg);
    }

    /// `AddrHandler::do_wait` will panic if the `Addr` for `RecipientWeak` is gone.
    #[inline]
    fn do_wait(&self, msg: M) {
        let addr = &self.upgrade().unwrap();
        Addr::do_wait(addr, msg);
    }
}

/// A trait object of `Addr<Actor>` that bind to given `Message` type
pub struct Recipient<RT, M: Message + Send>(Box<dyn AddrHandler<RT, M>>);

impl<RT, M: Message + Send> Deref for Recipient<RT, M> {
    type Target = dyn AddrHandler<RT, M>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

/// A trait object of `WeakAddr<Actor>` that bind to given `Message` type
pub struct RecipientWeak<RT, M: Message + Send>(Box<dyn AddrHandler<RT, M>>);

impl<RT, M: Message + Send> Deref for RecipientWeak<RT, M> {
    type Target = dyn AddrHandler<RT, M>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
