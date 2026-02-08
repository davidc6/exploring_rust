use std::{sync::{Arc, Mutex}, task::Waker};
use std::pin::Pin;
use std::task::{Context, Poll};

impl Countdown {
    fn new() -> Self {
        Countdown {
            completed: Arc::new(Mutex::new(false)),
            waker_stored: Arc::new(Mutex::new(None)),
            count: 0,
            started: false,
        }
    }
}

struct Countdown {
    completed: Arc<Mutex<bool>>,
    waker_stored: Arc<Mutex<Option<Waker>>>,
    count: u32,
    started: bool
}

struct InnerFut<'a> {
    inner: &'a Countdown
}

impl<'a> Future for InnerFut<'a> {
    type Output = &'a str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'a str> {
        // Check if the countdown reached 0
        if *self.inner.completed.lock().unwrap() {
            return Poll::Ready("Liftoff!");
        }

        // Waker gets store for the background thread to wake.
        *self.inner.waker_stored.lock().unwrap() = Some(cx.waker().clone());

        if !self.inner.started {
            // self.inner.started = true;

            let a = self.inner.completed.clone();
            let b = *a.lock().unwrap() = true;
            // a.inner.started = true;

            let completed = Arc::clone(&self.inner.completed);
            let waker = Arc::clone(&self.inner.waker_stored);
            let mut count = self.inner.count;

            if count == 0 {
                *completed.lock().unwrap() = true;

                if let Some(w) = waker.lock().unwrap().take() {
                    w.wake();
                }
            } else {
                count = count - 1;
            }
        }

        Poll::Pending
    }
}

fn main() {
    println!("Hello, world!");
}
