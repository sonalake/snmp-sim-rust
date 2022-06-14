/* Can be considered best practice for using this crate */

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use actix_async::prelude::*;
use futures_intrusive::sync::LocalMutex;
use futures_util::stream::{FuturesUnordered, StreamExt};
use tokio::time::sleep;

// use smart pointers wrapping your actor state for mutation and/or share purpose.
struct MyActor {
    state_mut: RefCell<usize>,
    state_shared: Rc<usize>,
    state_mut_await: LocalMutex<usize>,
}
actor!(MyActor);

struct Msg;
message!(Msg, ());

#[actix_async::handler]
impl Handler<Msg> for MyActor {
    // Use handle method whenever you can. Handler::handle_wait would always be slower.
    async fn handle(&self, _: Msg, _: Context<'_, Self>) {
        // RefCell can safely mutate actor state as long as RefMut is not held across await point.
        let mut state = self.state_mut.borrow_mut();

        *state += 1;

        // *. You must actively drop RefMut before await point. Otherwise the handle method
        // would  try to hold it for the entire scope. Leading to runtime borrow checker error.
        drop(state);

        sleep(Duration::from_millis(1)).await;

        // Rc can be cloned and used in spawned async tasks.
        let state = self.state_shared.clone();
        tokio::task::spawn_local(async move {
            sleep(Duration::from_millis(1)).await;
            drop(state);
        });

        // futures_intrusive::sync::LocalMutex is an async Mutex that is low cost and not
        // thread safe. It can hold mutable state across await point.
        //
        // *. This also applies to other async internal mutability primitives.
        // eg: tokio::sync::{Mutex, RwLock}
        let mut state = self.state_mut_await.lock().await;

        sleep(Duration::from_millis(1)).await;

        // We held the mutable state across await point. But it comes with a cost.
        // The actor would be blocked on this message as long as the MutexGuard is
        // held which means you lose concurrency completely in the whole process.
        //
        // So use async mutex as your last resort and try to avoid it whenever you can.
        *state += 1;
    }
}

#[actix_async::main]
async fn main() {
    let act = MyActor {
        state_mut: RefCell::new(0),
        state_shared: Rc::new(0),
        state_mut_await: LocalMutex::new(0, false),
    };

    let addr = act.start();

    let mut fut = FuturesUnordered::new();

    for _ in 0..256 {
        fut.push(addr.send(Msg));
    }

    while fut.next().await.is_some() {}

    let res = addr.stop(true).await;

    assert!(res.is_ok())
}
