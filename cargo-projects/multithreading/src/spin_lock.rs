use std::{cell::UnsafeCell, sync::atomic::AtomicBool, thread};

// ------------------
//
// SpinLock
//  - One of the simplest mutual exclusion mechanisms
//  - The idea is that the code keeps spinning/looping until is_locked == false is returned (until lock is acquired)
//  - Thread that keeps on trying to acquire the lock is put into a busy waiting/looping mode while waiting to acquire the lock
//  - Since the caller context hogs the CPU until the lock is acquired, it is important to not use within critical sections
//  - Although this mechanism is a resourceful, it works well when locked briefly
//
// ------------------

pub struct SpinLock<T> {
    is_locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            is_locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    // On the first lock(), locks the data.
    // On consecutive lock() call spins while the the data is still locked.
    pub fn lock(&self) -> &T {
        let id = thread::current().id();
        println!("LOCK requested by thread: {:?}", id);

        // Check/compare if current value is false sets to true
        // Or if current value is true, keep on trying to lock it.
        while self
            .is_locked
            .compare_exchange_weak(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err()
        {
            std::hint::spin_loop();
        }

        unsafe { &mut *self.value.get() }
    }

    // unlock() the data
    pub fn unlock(&self) {
        self.is_locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

// In order to share the data between threads we need to implement Sync on SpinLock.
// By doing this we tell the compiler that it is actually safe to
// share data between threads but we must only limit to types that are safe to send (Send).
unsafe impl<T> Sync for SpinLock<T> where T: Send {}
