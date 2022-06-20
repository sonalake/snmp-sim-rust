use core::future::{ready, Future};

use alloc::boxed::Box;

use super::address::Addr;
use super::context::Context;
use super::context_future::{ContextFuture, ContextInner};
use super::runtime::RuntimeService;
use super::util::{channel::channel, futures::LocalBoxFuture};

/// trait for stateful async actor.
pub trait Actor: Sized + 'static {
    /// actor is async and needs a runtime.
    type Runtime: RuntimeService;

    /// async hook before actor start to run.
    fn on_start<'act, 'ctx, 'res>(&'act mut self, ctx: Context<'ctx, Self>) -> LocalBoxFuture<'res, ()>
    where
        'act: 'res,
        'ctx: 'res,
    {
        Box::pin(async move {
            let _ = ctx;
        })
    }

    /// async hook before actor stops
    fn on_stop<'act, 'ctx, 'res>(&'act mut self, ctx: Context<'ctx, Self>) -> LocalBoxFuture<'res, ()>
    where
        'act: 'res,
        'ctx: 'res,
    {
        Box::pin(async move {
            let _ = ctx;
        })
    }

    #[cfg(feature = "tokio-rt")]
    /// A method called when supervisor detected a stopped actor future.
    ///
    /// Return `ActorState::Running` to re-construct the actor future and keep it running.
    /// Return everything else to stop the supervised actor future.
    fn supervised(mut state: super::supervisor::SupervisedState) -> ActorState {
        // take error from state and restart actor instance if it's a panic.
        match state.take_error() {
            Some(e) if e.is_panic() => ActorState::Running,
            _ => ActorState::Stop,
        }
    }

    /// start the actor on current thread and return it's address
    #[inline]
    fn start(self) -> Addr<Self> {
        Self::create(|_| self)
    }

    /// create actor with closure
    #[inline]
    fn create<F>(f: F) -> Addr<Self>
    where
        F: for<'c> FnOnce(Context<'c, Self>) -> Self + 'static,
    {
        Self::create_async(|ctx| ready(f(ctx)))
    }

    /// create actor with async closure
    /// # example:
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use actix_async::prelude::*;
    /// use actix_async::{actor, message};
    /// use futures_util::FutureExt;
    ///
    /// struct TestActor;
    /// actor!(TestActor);
    ///
    /// impl TestActor {
    ///     async fn test(&mut self) -> usize {
    ///         996
    ///     }
    /// }
    ///
    /// #[actix_async::main]
    /// async fn main() {
    ///     let addr = TestActor::create_async(|ctx| {
    ///         // *. notice context can not move to async block. so you have to use it from
    ///         // outside if you would.
    ///         let _ctx = ctx;
    ///         async {
    ///             // run async code
    ///             tokio::time::sleep(Duration::from_secs(1)).await;
    ///             // return an instance of actor.
    ///             TestActor
    ///         }
    ///     });
    ///     
    ///     // run async closure with actor and it's context.
    ///     let res = addr.run_wait(|act, ctx| act.test().boxed_local()).await;
    ///     assert_eq!(996, res.unwrap());
    /// }
    /// ```
    fn create_async<F, Fut>(f: F) -> Addr<Self>
    where
        F: for<'c> FnOnce(Context<'c, Self>) -> Fut + 'static,
        Fut: Future<Output = Self> + 'static,
    {
        let (tx, fut) = _create_context(f);

        <Self::Runtime as RuntimeService>::spawn(async move {
            let ctx_fut = fut.await;
            ctx_fut.run().await;
        });

        tx
    }

    /// create actor with async closure and expose it's [`ContextFuture`](super::context_future::ContextFuture).
    /// ContextFuture can be used to delay the start of actor and take control of when and where it would be polled.
    /// # example:
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use actix_async::prelude::*;
    /// use actix_async::{actor, message};
    /// use futures_util::FutureExt;
    ///
    /// struct TestActor;
    /// actor!(TestActor);
    ///
    /// impl TestActor {
    ///     async fn test(&mut self) -> usize {
    ///         996
    ///     }
    /// }
    ///
    /// #[actix_async::main]
    /// async fn main() {
    ///     let (addr, fut) = TestActor::create_context(|ctx| {
    ///         // *. notice context can not move to async block. so you have to use it from
    ///         // outside if you would.
    ///         let _ctx = ctx;
    ///         async {
    ///             // run async code
    ///             tokio::time::sleep(Duration::from_secs(1)).await;
    ///             // return an instance of actor.
    ///             TestActor
    ///         }
    ///     });
    ///     
    ///     // await on the fut to get ContextFuture.
    ///     let ctx_fut = fut.await;
    ///
    ///     // manual spawn ContextFuture to tokio runtime.     
    ///     tokio::task::spawn_local(async move {
    ///         ctx_fut.run().await;
    ///     });
    ///     
    ///     // run async closure with actor and it's context.
    ///     let res = addr.run_wait(|act, ctx| act.test().boxed_local()).await;
    ///     assert_eq!(996, res.unwrap());
    /// }
    /// ```
    fn create_context<F, Fut>(f: F) -> (Addr<Self>, LocalBoxFuture<'static, ContextFuture<Self>>)
    where
        F: for<'c> FnOnce(Context<'c, Self>) -> Fut + 'static,
        Fut: Future<Output = Self> + 'static,
    {
        let (tx, fut) = _create_context(f);

        (tx, Box::pin(fut))
    }

    /// capacity of the actor's channel and actor's task queue.
    ///
    /// Limit the max count of in flight messages and concurrent async tasks.
    ///
    /// Default to `256`.
    #[inline]
    fn size_hint() -> usize {
        256
    }
}

fn _create_context<A, F, Fut>(f: F) -> (Addr<A>, impl Future<Output = ContextFuture<A>>)
where
    A: Actor,
    F: for<'c> FnOnce(Context<'c, A>) -> Fut + 'static,
    Fut: Future<Output = A>,
{
    let (tx, rx) = channel(A::size_hint());

    let tx = Addr::new(tx);

    let ctx = ContextInner::new(rx);

    (tx, ContextFuture::start(f, ctx))
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ActorState {
    Running,
    Stop,
    StopGraceful,
}
