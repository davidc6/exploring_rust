use std::{cell::UnsafeCell, ops::Deref, sync::atomic::AtomicU32};
use std::sync::atomic::Ordering::{Acquire};
use atomic_wait::{wait};

struct Mutex<T> {
    /// Unlocked or locked, 0 or 1
    state: AtomicU32,
    /// Value that we'd like to store.
    /// UnsafeCell used here for interior mutability (core primitive).
    val: UnsafeCell<T>
}

// We should be able to share Mutex between threads.
// Therefore we implement Sync for Mutex.
// Sync and Send are unsafe to implement.
unsafe impl<T> Sync for Mutex<T> where T: Send {}

struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.val.get() }
    }
}

impl<T> Mutex<T> {
    pub const fn new(val: T) -> Self {
        Mutex {
            state: AtomicU32::new(0),
            val: UnsafeCell::new(val)
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        while self.state.swap(1, Acquire) == 1 {
            wait(&self.state, 1);
        }

        MutexGuard {
            mutex: self
        }
    }

}

fn main() {
    let m = Mutex::new(0);
    let _l = m.lock();
    let d = *_l;
    println!("{d}");
}
