use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn func() {
    // Using Acquire-Release memory ordering here,
    // makes sure that it forms a happens before relationship
    // between unlocking the lock and locking it.
    // All the memory operations happen inside of this 
    // barrier sandwich preventing any reordering operations across 
    // the boundaries.
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
        // Safety: Exclusive lock hold. Nothing else is accessing DATA.
        unsafe { 
            DATA = String::from('!') 
        };

        LOCKED.store(false, Release);
    }
}
 
fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(func);
        }
    });
}
