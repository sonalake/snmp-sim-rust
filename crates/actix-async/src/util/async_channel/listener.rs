/// Copy/paste from event-listener crate
/// The goal is to make blocking listener that would need std features optional.
use core::{
    cell::{Cell, UnsafeCell},
    fmt,
    future::Future,
    mem::{self, ManuallyDrop},
    ops::{Deref, DerefMut},
    pin::Pin,
    ptr::{self, NonNull},
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
    task::Waker,
    task::{Context, Poll},
};

use alloc::boxed::Box;

use crate::util::smart_pointer::{Lock, LockGuard, RefCounter};

use super::full_fence;

struct Inner {
    notified: AtomicUsize,
    list: Lock<List>,
    cache: UnsafeCell<Entry>,
}

impl Inner {
    fn lock(&self) -> ListGuard<'_> {
        ListGuard {
            inner: self,
            guard: self.list.lock(),
        }
    }

    #[inline(always)]
    fn cache_ptr(&self) -> NonNull<Entry> {
        unsafe { NonNull::new_unchecked(self.cache.get()) }
    }
}

pub(super) struct Event {
    inner: AtomicPtr<Inner>,
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

impl Event {
    #[inline]
    pub const fn new() -> Event {
        Event {
            inner: AtomicPtr::new(ptr::null_mut()),
        }
    }

    #[cold]
    pub fn listen(&self) -> EventListener {
        let inner = self.inner();
        let listener = EventListener {
            inner: unsafe { RefCounter::clone(&ManuallyDrop::new(RefCounter::from_raw(inner))) },
            entry: Some(inner.lock().insert(inner.cache_ptr())),
        };

        // Make sure the listener is registered before whatever happens next.
        full_fence();
        listener
    }

    #[inline]
    pub fn notify(&self, n: usize) {
        // Make sure the notification comes after whatever triggered it.
        full_fence();

        if let Some(inner) = self.try_inner() {
            // Notify if there is at least one unnotified listener and the number of notified
            // listeners is less than `n`.
            if inner.notified.load(Ordering::Acquire) < n {
                inner.lock().notify(n);
            }
        }
    }

    #[inline]
    fn try_inner(&self) -> Option<&Inner> {
        let inner = self.inner.load(Ordering::Acquire);
        unsafe { inner.as_ref() }
    }

    fn inner(&self) -> &Inner {
        let mut inner = self.inner.load(Ordering::Acquire);

        // Initialize the state if this is its first use.
        if inner.is_null() {
            // Allocate on the heap.
            let new = RefCounter::new(Inner {
                notified: AtomicUsize::new(usize::MAX),
                list: Lock::new(List {
                    head: None,
                    tail: None,
                    start: None,
                    len: 0,
                    notified: 0,
                    cache_used: false,
                }),
                cache: UnsafeCell::new(Entry {
                    state: Cell::new(State::Created),
                    prev: Cell::new(None),
                    next: Cell::new(None),
                }),
            });
            // Convert the heap-allocated state into a raw pointer.
            let new = RefCounter::into_raw(new) as *mut Inner;

            // Attempt to replace the null-pointer with the new state pointer.
            inner = self
                .inner
                .compare_exchange(inner, new, Ordering::AcqRel, Ordering::Acquire)
                .unwrap_or_else(|x| x);

            // Check if the old pointer value was indeed null.
            if inner.is_null() {
                // If yes, then use the new state pointer.
                inner = new;
            } else {
                // If not, that means a concurrent operation has initialized the state.
                // In that case, use the old pointer and deallocate the new one.
                unsafe {
                    drop(RefCounter::from_raw(new));
                }
            }
        }

        unsafe { &*inner }
    }
}

impl Drop for Event {
    #[inline]
    fn drop(&mut self) {
        let inner: *mut Inner = *self.inner.get_mut();

        // If the state pointer has been initialized, deallocate it.
        if !inner.is_null() {
            unsafe {
                drop(RefCounter::from_raw(inner));
            }
        }
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Event { .. }")
    }
}

impl Default for Event {
    fn default() -> Event {
        Event::new()
    }
}

pub struct EventListener {
    inner: RefCounter<Inner>,
    entry: Option<NonNull<Entry>>,
}

unsafe impl Send for EventListener {}
unsafe impl Sync for EventListener {}

impl fmt::Debug for EventListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("EventListener { .. }")
    }
}

impl Future for EventListener {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut list = self.inner.lock();

        let entry = match self.entry {
            None => unreachable!("cannot poll a completed `EventListener` future"),
            Some(entry) => entry,
        };
        let state = unsafe { &entry.as_ref().state };

        // Do a dummy replace operation in order to take out the state.
        match state.replace(State::Notified(false)) {
            State::Notified(_) => {
                // If this listener has been notified, remove it from the list and return.
                list.remove(entry, self.inner.cache_ptr());
                drop(list);
                self.entry = None;
                return Poll::Ready(());
            }
            State::Created => {
                // If the listener was just created, put it in the `Polling` state.
                state.set(State::Polling(cx.waker().clone()));
            }
            State::Polling(w) => {
                // If the listener was in the `Polling` state, update the waker.
                if w.will_wake(cx.waker()) {
                    state.set(State::Polling(w));
                } else {
                    state.set(State::Polling(cx.waker().clone()));
                }
            }
        }

        Poll::Pending
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        // If this listener has never picked up a notification...
        if let Some(entry) = self.entry.take() {
            let mut list = self.inner.lock();

            // But if a notification was delivered to it...
            if let State::Notified(additional) = list.remove(entry, self.inner.cache_ptr()) {
                // Then pass it on to another active listener.
                if additional {
                    list.notify_additional(1);
                } else {
                    list.notify(1);
                }
            }
        }
    }
}

struct ListGuard<'a> {
    inner: &'a Inner,
    guard: LockGuard<'a, List>,
}

impl Drop for ListGuard<'_> {
    #[inline]
    fn drop(&mut self) {
        let list = &mut **self;

        // Update the atomic `notified` counter.
        let notified = if list.notified < list.len {
            list.notified
        } else {
            usize::MAX
        };
        self.inner.notified.store(notified, Ordering::Release);
    }
}

impl Deref for ListGuard<'_> {
    type Target = List;

    #[inline]
    fn deref(&self) -> &List {
        &*self.guard
    }
}

impl DerefMut for ListGuard<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut List {
        &mut *self.guard
    }
}

enum State {
    Created,
    Notified(bool),
    Polling(Waker),
}

impl State {
    #[inline]
    fn is_notified(&self) -> bool {
        match self {
            State::Notified(_) => true,
            State::Created | State::Polling(_) => false,
        }
    }
}

struct Entry {
    state: Cell<State>,
    prev: Cell<Option<NonNull<Entry>>>,
    next: Cell<Option<NonNull<Entry>>>,
}

struct List {
    #[allow(dead_code)]
    head: Option<NonNull<Entry>>,
    tail: Option<NonNull<Entry>>,
    start: Option<NonNull<Entry>>,
    len: usize,
    notified: usize,
    cache_used: bool,
}

impl List {
    fn insert(&mut self, cache: NonNull<Entry>) -> NonNull<Entry> {
        unsafe {
            let entry = Entry {
                state: Cell::new(State::Created),
                prev: Cell::new(self.tail),
                next: Cell::new(None),
            };

            let entry = if self.cache_used {
                // Allocate an entry that is going to become the new tail.
                NonNull::new_unchecked(Box::into_raw(Box::new(entry)))
            } else {
                // No need to allocate - we can use the cached entry.
                self.cache_used = true;
                cache.as_ptr().write(entry);
                cache
            };

            // Replace the tail with the new entry.
            match mem::replace(&mut self.tail, Some(entry)) {
                None => self.head = Some(entry),
                Some(t) => t.as_ref().next.set(Some(entry)),
            }

            // If there were no unnotified entries, this one is the first now.
            if self.start.is_none() {
                self.start = self.tail;
            }

            // Bump the entry count.
            self.len += 1;

            entry
        }
    }

    fn remove(&mut self, entry: NonNull<Entry>, cache: NonNull<Entry>) -> State {
        unsafe {
            let prev = entry.as_ref().prev.get();
            let next = entry.as_ref().next.get();

            // Unlink from the previous entry.
            match prev {
                None => self.head = next,
                Some(p) => p.as_ref().next.set(next),
            }

            // Unlink from the next entry.
            match next {
                None => self.tail = prev,
                Some(n) => n.as_ref().prev.set(prev),
            }

            // If this was the first unnotified entry, move the pointer to the next one.
            if self.start == Some(entry) {
                self.start = next;
            }

            // Extract the state.
            let state = if ptr::eq(entry.as_ptr(), cache.as_ptr()) {
                // Free the cached entry.
                self.cache_used = false;
                entry.as_ref().state.replace(State::Created)
            } else {
                // Deallocate the entry.
                Box::from_raw(entry.as_ptr()).state.into_inner()
            };

            // Update the counters.
            if state.is_notified() {
                self.notified -= 1;
            }
            self.len -= 1;

            state
        }
    }

    #[cold]
    fn notify(&mut self, mut n: usize) {
        if n <= self.notified {
            return;
        }
        n -= self.notified;

        while n > 0 {
            n -= 1;

            // Notify the first un notified entry.
            match self.start {
                None => break,
                Some(e) => {
                    // Get the entry and move the pointer forward.
                    let e = unsafe { e.as_ref() };
                    self.start = e.next.get();

                    // Set the state of this entry to `Notified` and notify.
                    match e.state.replace(State::Notified(false)) {
                        State::Notified(_) => {}
                        State::Created => {}
                        State::Polling(w) => w.wake(),
                    }

                    // Update the counter.
                    self.notified += 1;
                }
            }
        }
    }

    #[cold]
    fn notify_additional(&mut self, mut n: usize) {
        while n > 0 {
            n -= 1;

            // Notify the first un notified entry.
            match self.start {
                None => break,
                Some(e) => {
                    // Get the entry and move the pointer forward.
                    let e = unsafe { e.as_ref() };
                    self.start = e.next.get();

                    // Set the state of this entry to `Notified` and notify.
                    match e.state.replace(State::Notified(true)) {
                        State::Notified(_) => {}
                        State::Created => {}
                        State::Polling(w) => w.wake(),
                    }

                    // Update the counter.
                    self.notified += 1;
                }
            }
        }
    }
}
