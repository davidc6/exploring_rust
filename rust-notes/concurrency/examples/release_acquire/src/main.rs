use std::sync::atomic::{AtomicBool, AtomicU64}; 
use std::thread;
use std::time::Duration;
use std::sync::atomic::Ordering::{Relaxed, Release, Acquire};

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

/*
    Acquire - Release operations form the happens-before relationship between atomic operations. 
    The sequence of the below operations should be:

    - load "false" from READY (Acquire)
        - (on thread 2 slightly after the above operation) store "123" in Data
    - load "false" from READY (Acquire)
        - (on thread 2 slightly after the above operation) store "true" in Ready
    - load "true" from READY (Acquire)
    - load 123 from Data 
*/
fn main() {
    // Spawn a new thread and send data to the main thread.
    // Store 123 in DATA using Relaxed ordering.
    // Store true in READY using Release ordering.
    // Release ordering applies to load operations.
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        // We use this to indicate to the main thread that 
        // the data has been stored and ready to be read.
        READY.store(true, Release);
    });

    // Everything before the "Release" store is visible
    // once READY loads true.
    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("Waiting...");
    }

    println!("{}", DATA.load(Relaxed));
}

