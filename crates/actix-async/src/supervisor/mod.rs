mod error;
mod worker;

pub use tokio::task::JoinError;

pub(super) use self::error::SupervisorError;

use core::{future::Future, mem, time::Duration};

use tokio::{runtime::Handle, select};

use super::actor::{Actor, ActorState};
use super::address::Addr;
use super::context::Context;
use super::context_future::{ContextFuture, ContextInner};
use super::util::{
    channel::{channel, Sender},
    futures::BoxFuture,
    smart_pointer::{Lock, RefCounter},
};

use self::worker::Worker;

/// A supervisor can be used to manage multiple actors and/or multiple instances of the same actor
/// type on a threaded tokio runtime.
///
/// # Examples
///
/// ```rust
/// # use actix_async::{prelude::*, supervisor::Supervisor};
/// # use futures_util::FutureExt;
///
/// // actor type and impl.
/// struct MyActor;
/// actor!(MyActor);
///
/// impl MyActor {
///     async fn handle(&self, ctx: Context<'_, Self>) -> usize {
///         996
///     }
/// }
///
/// // start a multi-thread tokio runtime
/// #[tokio::main]
/// async fn main() {
///     // construct a supervisor with 2 worker threads.
///     let supervisor = Supervisor::builder().workers(2).build();
///     // start one instance of actor.
///     let addr = supervisor.start(1, |_| async { MyActor }).await;
///
///     // run a closure with the started address
///     let res = addr.run(|act, ctx| act.handle(ctx).boxed_local()).await.unwrap();
///
///     assert_eq!(res, 996);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Supervisor {
    join_handles: RefCounter<Lock<Vec<Worker>>>,
    tx: Sender<BoxFuture<'static, ()>>,
    shutdown_timeout: Duration,
}

impl Supervisor {
    pub fn builder() -> SupervisorBuilder {
        SupervisorBuilder::new()
    }

    /// Stop the supervisor and remove all resource it uses.
    ///
    /// When is_graceful is set to true the stop process would wait for all actors future
    /// stop.
    /// (By [`Addr::stop`](crate::address::Addr::stop) or any other means that render
    /// an actor future exit.)
    ///
    /// The progress of graceful shutdown would last until it reaches shutdown_timeout of
    /// Supervisor.
    pub async fn stop(self, is_graceful: bool) -> bool {
        if self.tx.close() {
            let handles = mem::take(&mut *self.join_handles.lock());

            if handles.is_empty() {
                false
            } else {
                let stop = Box::pin(async move {
                    for handle in handles {
                        let _ = handle.stop(is_graceful).await;
                    }
                });

                let timeout = Box::pin(tokio::time::sleep(self.shutdown_timeout));

                select! {
                    _ = stop => true,
                    _ = timeout => false
                }
            }
        } else {
            false
        }
    }

    pub async fn start<F, Fut, A>(&self, num: usize, func: F) -> Addr<A>
    where
        F: for<'c> Fn(Context<'c, A>) -> Fut + Clone + Send + 'static,
        Fut: Future<Output = A> + 'static,
        A: Actor,
    {
        let (tx, rx) = channel(A::size_hint());

        let addr = Addr::new(tx);

        for _ in 0..num {
            let rx = rx.clone();
            let func = func.clone();

            // TODO: handle error.
            let _ = self
                .tx
                .send(Box::pin(async move {
                    loop {
                        let func = func.clone();
                        let rx_clone = rx.clone();
                        let handle = tokio::task::spawn_local(async move {
                            let ctx = ContextInner::new(rx_clone);
                            let fut = ContextFuture::start(func, ctx).await;
                            fut.run().await
                        });

                        let res = handle.await;

                        let state = SupervisedState { error: res.err() };

                        match A::supervised(state) {
                            ActorState::Running if !rx.is_closed() => continue,
                            _ => break,
                        }
                    }
                }) as _)
                .await;
        }

        addr
    }
}

impl Drop for Supervisor {
    fn drop(&mut self) {
        // Check if is the last copy of supervisor.
        if RefCounter::strong_count(&self.join_handles) == 1 {
            // close channel in case supervisor is not closed by Supervisor::stop
            let _ = self.tx.close();

            // if handles are still there just drop them.
            // Worker would force a recovery of resource on drop.
            let worker_handles = mem::take(&mut *self.join_handles.lock());
            drop(worker_handles);
        }
    }
}

pub struct SupervisorBuilder {
    workers: usize,
    shutdown_timeout: Duration,
}

impl Default for SupervisorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SupervisorBuilder {
    pub fn new() -> Self {
        SupervisorBuilder {
            workers: 4,
            shutdown_timeout: Duration::from_secs(30),
        }
    }

    /// Change the worker threads a supervisor would spawn.
    ///
    /// Default to 4.
    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    /// Change the graceful shutdown timeout. Supervisor would be force to stop
    /// after this period of time.
    ///
    /// Default to 30 seconds.
    pub fn shutdown_timeout(mut self, dur: Duration) -> Self {
        self.shutdown_timeout = dur;
        self
    }

    pub fn build(self) -> Supervisor {
        let (tx, rx) = channel(self.workers);

        let mut workers = Vec::new();

        for _ in 0..self.workers {
            let rx = rx.clone();
            let handle = Handle::current();

            let worker = Worker::new(rx, handle);

            workers.push(worker);
        }

        Supervisor {
            join_handles: RefCounter::new(Lock::new(workers)),
            tx,
            shutdown_timeout: self.shutdown_timeout,
        }
    }
}

/// peek into the output of supervised actor future after it finished.
pub struct SupervisedState {
    error: Option<JoinError>,
}

impl SupervisedState {
    /// Take the error if actor future exit with error.
    pub fn take_error(&mut self) -> Option<JoinError> {
        self.error.take()
    }
}
