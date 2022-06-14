// A benchmark for the runtime cost of actor pattern.
//
// * This is mainly focus on how well an actor can schedule multiple async tasks in concurrent.
// Time count how long all tasks are finished. Polled count how many polls are done by runtime.
//
// actix-async would respect the rule of rust future and only wake up task that can make progress
// so it has close to minimal polling times for resolving a future.

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{self, Poll};
use std::time::{Duration, Instant};

use futures_util::stream::{FuturesUnordered, StreamExt};
use tokio::time::sleep;

// A counter for counting times futures are polled in global scope.
static POLL: AtomicUsize = AtomicUsize::new(0);

fn main() -> std::io::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(tokio::task::LocalSet::new().run_until(async {
            let act = MyActor { state: Cell::new(0) };

            let addr = actix_async::prelude::Actor::start(act);

            let mut fut = FuturesUnordered::new();

            for _ in 0..999 {
                fut.push(addr.send(Msg));
            }

            let now = Instant::now();

            while fut.next().await.is_some() {}

            println!(
                "actix-async bench finished.\r\nTotal time used: {:?}\r\nTotal polled times: {}\r\n",
                now.elapsed(),
                POLL.load(Ordering::Relaxed)
            );

            let res = addr.stop(true).await;

            assert!(res.is_ok());
        }));

    POLL.store(0, Ordering::SeqCst);

    actix::System::new().block_on(async {
        let act = MyActor { state: Cell::new(0) };

        let addr = actix::Actor::start(act);

        let mut fut = FuturesUnordered::new();

        for _ in 0..999 {
            fut.push(addr.send(Msg));
        }

        let now = Instant::now();

        while fut.next().await.is_some() {}

        println!(
            "actix bench finished.\r\nTotal time used: {:?}\r\nTotal polled times: {}",
            now.elapsed(),
            POLL.load(Ordering::Relaxed)
        );
    });

    Ok(())
}

struct MyActor {
    state: Cell<u64>,
}

struct Msg;

mod impl_actix_async {
    use super::*;

    use actix_async::prelude::*;

    impl Actor for MyActor {
        type Runtime = TokioRuntime;

        // actix-async respect capacity of mailbox strictly.
        // use size_hint to make it can run all 999 tasks concurrently.
        fn size_hint() -> usize {
            999
        }
    }

    message!(Msg, ());

    #[actix_async::handler]
    impl Handler<Msg> for MyActor {
        async fn handle(&self, _: Msg, _: Context<'_, Self>) {
            let state = self.state.get() + 1;
            self.state.set(state);

            CountedFuture(sleep(Duration::from_millis(state))).await;
        }
    }
}

mod impl_actix {
    use super::*;

    use actix::prelude::*;

    impl actix::Actor for MyActor {
        type Context = actix::Context<Self>;
    }

    impl actix::Message for Msg {
        type Result = ();
    }

    impl actix::Handler<Msg> for MyActor {
        type Result = actix::ResponseActFuture<Self, ()>;

        fn handle(&mut self, _: Msg, _: &mut Self::Context) -> Self::Result {
            let state = self.state.get() + 1;
            self.state.set(state);

            CountedFuture(sleep(Duration::from_millis(state)))
                .into_actor(self)
                .boxed_local()
        }
    }
}

struct CountedFuture<Fut>(Fut);

impl<Fut> Future for CountedFuture<Fut>
where
    Fut: Future,
{
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        POLL.fetch_add(1, Ordering::Relaxed);
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) }.poll(cx)
    }
}
