use std::sync::atomic::{fence, AtomicBool, Ordering};
use std::sync::atomic::Ordering::{Release, Relaxed, Acquire};

pub struct Mutex {
    flag: AtomicBool
}

impl Mutex {
    pub fn new() -> Self {
        Mutex {
            flag: AtomicBool::new(false)
        }
    }

    pub fn lock(&self) {
        while self
            .flag
            .compare_exchange_weak(false, true, Relaxed, Relaxed)
            .is_err()

        {}

        // Sync with unlock()
        fence(Acquire);
    }

    pub fn unlock(&self) {
        self.flag.store(false, Release);
    }
}

fn main() {

    println!("Hello, world!");
}
