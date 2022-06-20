use core::{ops::Deref, task::Waker};

use alloc::{collections::VecDeque, task::Wake};

use super::util::smart_pointer::{Lock, RefCounter};

#[derive(Clone)]
pub(crate) struct ActorWaker {
    queue: WakeQueue,
    idx: usize,
    waker: Waker,
}

impl ActorWaker {
    pub(crate) fn new(queued: &WakeQueue, idx: usize, waker: &Waker) -> RefCounter<Self> {
        RefCounter::new(Self {
            queue: WakeQueue::clone(queued),
            idx,
            waker: Waker::clone(waker),
        })
    }
}

impl Wake for ActorWaker {
    fn wake(self: RefCounter<Self>) {
        self.wake_by_ref()
    }

    fn wake_by_ref(self: &RefCounter<Self>) {
        let ActorWaker {
            ref queue,
            ref idx,
            ref waker,
        } = **self;

        queue.enqueue(*idx);

        waker.wake_by_ref();
    }
}

#[derive(Clone)]
pub(crate) struct WakeQueue(RefCounter<Lock<VecDeque<usize>>>);

impl Deref for WakeQueue {
    type Target = Lock<VecDeque<usize>>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl WakeQueue {
    #[inline]
    pub(crate) fn new() -> Self {
        Self(RefCounter::new(Lock::new(VecDeque::new())))
    }

    #[inline]
    pub(crate) fn enqueue(&self, idx: usize) {
        self.lock().push_back(idx);
    }
}
