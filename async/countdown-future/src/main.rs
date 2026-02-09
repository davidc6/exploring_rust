use std::{sync::{Arc, Mutex}, task::Waker};
use std::pin::Pin;
use std::task::{Context, Poll};

impl Countdown {
    fn new(count: u32) -> Self {
        Countdown {
            completed: Arc::new(Mutex::new(false)),
            waker_stored: Arc::new(Mutex::new(None)),
            count: Arc::new(Mutex::new(count)),
            started: Arc::new(Mutex::new(false)),
        }
    }
}

struct Countdown {
    completed: Arc<Mutex<bool>>,
    waker_stored: Arc<Mutex<Option<Waker>>>,
    count: Arc<Mutex<u32>>,
    started: Arc<Mutex<bool>>,
}

struct InnerFut<'a> {
    inner: &'a Countdown
}

impl<'a> Future for InnerFut<'a> {
    type Output = &'a str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the countdown reached 0
        if *self.inner.completed.lock().unwrap() {
            return Poll::Ready("Liftoff!");
        }

        // Waker gets store for the background thread to wake.
        *self.inner.waker_stored.lock().unwrap() = Some(cx.waker().clone());

        if !*self.inner.started.lock().unwrap() {
            let mut started_lock = self.inner.started.lock().unwrap();
            *started_lock = true;

            let completed = Arc::clone(&self.inner.completed);
            let waker = Arc::clone(&self.inner.waker_stored);

            if *self.inner.count.lock().unwrap() == 0 {
                *completed.lock().unwrap() = true;

                return Poll::Ready("Liftoff");
            } else {
                *self.inner.count.lock().unwrap() -= 1;
                
                if let Some(w) = waker.lock().unwrap().take() {
                    w.wake_by_ref();
                }
            }
        } else {
            let completed = Arc::clone(&self.inner.completed);
            let waker = Arc::clone(&self.inner.waker_stored);

            if *self.inner.count.lock().unwrap() == 0 {
                *completed.lock().unwrap() = true;

                if let Some(w) = waker.lock().unwrap().take() {
                    w.wake();
                }  
            } else {
                *self.inner.count.lock().unwrap() -= 1;
            }
        }

        Poll::Pending
    }
}

fn main() {
    let countdown = Countdown::new(2);
    let inner_fut = InnerFut {
        inner: &countdown
    };
    // inner_fut.await;
}
