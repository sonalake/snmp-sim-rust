mod async_channel;
mod async_oneshot;

pub(crate) mod channel {
    pub(crate) use super::async_channel::{channel, Receiver, SendFuture, Sender, WeakSender};
    pub(crate) use super::async_oneshot::{oneshot, OneshotReceiver, OneshotSender};
}

pub(crate) mod futures;

pub(crate) mod smart_pointer {
    use alloc::sync::{Arc, Weak};

    #[cfg(feature = "std")]
    use std::sync::{Mutex, MutexGuard};

    #[cfg(not(feature = "std"))]
    use spin::{Mutex, MutexGuard};

    pub(crate) type RefCounter<T> = Arc<T>;
    pub(crate) type WeakRefCounter<T> = Weak<T>;

    #[derive(Debug)]
    pub(crate) struct Lock<T>(Mutex<T>);

    pub(crate) type LockGuard<'g, T> = MutexGuard<'g, T>;

    impl<T> Lock<T> {
        pub(crate) fn new(t: T) -> Self {
            Self(Mutex::new(t))
        }
    }

    #[cfg(feature = "std")]
    impl<T> Lock<T> {
        pub(crate) fn lock(&self) -> LockGuard<'_, T> {
            self.0.lock().unwrap()
        }

        pub(crate) fn try_lock(&self) -> Option<LockGuard<'_, T>> {
            self.0.try_lock().ok()
        }
    }

    #[cfg(not(feature = "std"))]
    impl<T> Lock<T> {
        pub(crate) fn lock(&self) -> LockGuard<'_, T> {
            self.0.lock()
        }

        pub(crate) fn try_lock(&self) -> Option<LockGuard<'_, T>> {
            self.0.try_lock()
        }
    }
}
