use actix_async::prelude::*;
use actix_async::supervisor::Supervisor;
use futures_util::FutureExt;
use std::panic;

/*
   actix-async can run in context of multi threaded tokio runtime with help of supervisor.
*/

struct MyActor;
actor!(MyActor);

impl MyActor {
    async fn panic(&self) {
        panic!("intentional");
    }

    async fn ping(&self) -> usize {
        996
    }
}

struct Msg;
message!(Msg, ());

#[actix_async::handler]
impl Handler<Msg> for MyActor {
    async fn handle(&self, _: Msg, _: Context<'_, Self>) {
        tokio::task::spawn_blocking(|| println!("spawn_blocking works"))
            .await
            .unwrap();

        let spawn = tokio::spawn(async {
            tokio::task::block_in_place(|| println!("block_in_place works"));
            println!("spawn works");
        });
        let local = tokio::task::spawn_local(async {
            println!("spawn local works");
        });

        spawn.await.unwrap();
        local.await.unwrap();
    }
}

// start a multi-thread tokio runtime.
#[tokio::main(worker_threads = 4)]
async fn main() {
    // construct a supervisor with 2 worker threads.
    let supervisor = Supervisor::builder().workers(2).build();

    // start 1 instance of MyActor in supervisor.
    let addr = supervisor.start(1, |_| async { MyActor }).await;

    addr.send(Msg).await.unwrap();

    // make actor panic.
    let res = addr.run(|act, _| act.panic().boxed_local()).await;

    assert!(res.is_err());

    // actor is restarted.
    let res = addr.run(|act, _| act.ping().boxed_local()).await.unwrap();

    assert_eq!(res, 996);

    addr.stop(true).await.unwrap();

    // explicit stop supervisor is suggested.
    assert!(supervisor.stop(true).await)
}
