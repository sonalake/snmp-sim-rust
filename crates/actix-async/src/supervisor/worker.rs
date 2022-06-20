use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use std::thread::JoinHandle;

use tokio::{runtime::Handle, sync::Semaphore, task::LocalSet};

use crate::util::{
    channel::{oneshot, OneshotReceiver, OneshotSender, Receiver},
    futures::{ready, BoxFuture, Stream},
    smart_pointer::RefCounter,
};

use super::error::SupervisorError;

/// Worker runs on a spawned thread and handle
#[derive(Debug)]
pub(super) struct Worker {
    join_handle: Option<JoinHandle<()>>,
    limit: RefCounter<Semaphore>,
    exit_tx: Option<OneshotSender<()>>,
}

// See `tokio::sync::batch_semaphore::Semaphore::MAX_PERMITS` for reason.
const LIMIT: u32 = u32::MAX >> 3;

impl Worker {
    pub(super) fn new(rx: Receiver<BoxFuture<'static, ()>>, tokio_handle: Handle) -> Self {
        // use a semaphore to track spawned actors on every worker.
        let limit = RefCounter::new(Semaphore::new(LIMIT as usize));

        // a one shot channel to notify worker future to exit and despawn worker thread.
        let (exit_tx, exit_rx) = oneshot();

        let limit_clone = limit.clone();
        let join_handle = std::thread::spawn(move || {
            tokio_handle.block_on(LocalSet::new().run_until(WorkerFuture {
                rx,
                limit: limit_clone,
                exit_rx,
            }))
        });

        Self {
            join_handle: Some(join_handle),
            limit,
            exit_tx: Some(exit_tx),
        }
    }

    pub(super) async fn stop(mut self, is_graceful: bool) -> Result<(), SupervisorError> {
        if is_graceful {
            let _res = self.limit.acquire_many(LIMIT).await?;
        }

        let _ = self.exit_tx.take().unwrap().send(());

        self.join_handle.take().unwrap().join()?;

        Ok(())
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        // In case worker is not stopped with
        if let Some(exit_tx) = self.exit_tx.take() {
            let _ = exit_tx.send(());
        }
    }
}

struct WorkerFuture {
    rx: Receiver<BoxFuture<'static, ()>>,
    limit: RefCounter<Semaphore>,
    exit_rx: OneshotReceiver<()>,
}

impl Future for WorkerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match ready!(Pin::new(&mut this.rx).poll_next(cx)) {
            Some(msg) => {
                // TODO: This is not likely to reach limit but it should be properly implemented.
                let permit = this.limit.clone().try_acquire_owned().unwrap();
                tokio::task::spawn_local(async move {
                    msg.await;
                    drop(permit);
                });
                // be fair and yield. this would give other worker threads a chance to grab
                // the next actor future.
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            // channel is closed. wait for exit signal.
            None => Pin::new(&mut this.exit_rx).poll(cx).map(|_| ()),
        }
    }
}
