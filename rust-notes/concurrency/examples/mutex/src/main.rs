use std::{cell::UnsafeCell, ops::Deref, sync::atomic::AtomicU32};
use std::sync::atomic::Ordering::{Acquire, Release};
use atomic_wait::{wait, wake_one};

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

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // Sets state to unlocked.
        self.mutex.state.store(0, Release);
        // Wakes up a single thread that is waiting.
        // There could be multiple threads waiting but waking up
        // one is enough. 
        //
        // Wake and Wait are optimisations to avoid busy looping.
        wake_one(&self.mutex.state);
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
        // Acquire memory ordering makes the store part of the operation Relaxed.
        // swap(val, ordering) - returns previous value and sets new one.
        // Initially self.state gets set to 1 if 0.
        // Then while it's 1, waits atomically.
        while self.state.swap(1, Acquire) == 1 {
            // Atomically wait for the value of an atomic object to change.
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
