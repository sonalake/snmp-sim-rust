use std::time::{Duration, Instant};

use actix_async::prelude::*;
use actix_async::supervisor::Supervisor;
use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;

/*
   actix-async does not provide blocking actor feature.

   This example shows how to use supervisor to run a pool of actor that run blocking jobs in async
   context.

   It has more overhead than pure blocking thread pool due to the cost of async runtime.
   In exchange the design of lib is simpler and have a good mix usage of blocking and async.
*/

struct BlockingActor;
actor!(BlockingActor);

struct Msg;
message!(Msg, ());

#[actix_async::handler]
impl Handler<Msg> for BlockingActor {
    async fn handle(&self, _: Msg, _: Context<'_, Self>) {
        unimplemented!()
    }

    // since we are running pure blocking code. There is no point using concurrent handler
    // at all. use handle_wait would do just fine.
    async fn handle_wait(&mut self, _: Msg, _: Context<'_, Self>) {
        // use sleep to simulate heavy blocking computation.
        std::thread::sleep(Duration::from_millis(1));

        println!("running task on thread {:?}", std::thread::current().id());
    }
}

// actix-async supervisor can make use of multi-threaded tokio runtime.
#[tokio::main]
async fn main() {
    // construct a supervisor with 2 worker threads.
    let supervisor = Supervisor::builder().workers(2).build();

    // This is a scope to drop Addr<BlockingActor> early before calling Supervisor::stop.
    {
        // start 2 instance of BlockingActor in supervisor.
        let addr = supervisor.start(2, |_| async { BlockingActor }).await;

        // send 200 messages concurrently.
        let mut fut = FuturesUnordered::new();
        for _ in 0..200 {
            fut.push(addr.wait(Msg));
        }

        let now = Instant::now();
        while fut.next().await.is_some() {}

        // since we have 2 workers for 1 ms blocking job. the total time taken should be slightly above
        // 100 ms.
        //
        // *. There is a chance two actors end up on the same thread so the work would take
        // more than 200 ms to finish.
        // This is due to work stealing nature of the supervisor.
        //
        // It's working as intended as the work stealing can detect work load difference between
        // supervisor workers and adjust the distribution of actors.
        println!("\r\n\r\nTook {:?} to finish example", now.elapsed());
    }

    // explicit stop supervisor is suggested.
    assert!(supervisor.stop(true).await)
}
