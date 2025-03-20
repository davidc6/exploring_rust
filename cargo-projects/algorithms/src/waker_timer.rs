use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// This is the shared state between the future and waiting thread
struct SharedState {
    /// Has the sleep time elapsed
    has_completed: bool,

    /// Waker for the task that TimerFuture runs on.
    /// After setting has_completed to true,
    /// the thread can use this to tell
    /// TimerFuture to wake up and move forward.
    waker: Option<Waker>,
}

/// Future is the fundamental building block for async Rust.
/// We implement it on a type that want to poll().
impl Future for TimerFuture {
    // Unit type
    type Output = ();

    // Allows us to check if the value is available.
    // When the response is available then it is returned in a wrapped Poll::Ready(T)
    // Poll::Pending signals to the caller that the valus is not available yet.
    //
    // Pin - similar to &mut Self but pinned to a memory location
    // Context - is to pass a Waker to the async task (Timer). The waker
    // allows the async task to signal that it finished (timer is complete).
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Get the lock to the shared state.
        let mut shared_state = self.shared_state.lock().unwrap();

        // Check if the timer has completed, and if it has then
        // return the result (in this case it's just a Unit)
        if shared_state.has_completed {
            Poll::Ready(())
        } else {
            // If the timer hasn't completed yet,
            // the waker is set such that the thread can wake up the current task
            // when the time completes which makes sure that the future is polled again.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        // Create new shared state
        let shared_state = Arc::new(Mutex::new(SharedState {
            has_completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();

        // Spawn a new thread
        thread::spawn(move || {
            thread::sleep(duration);

            let mut shared_state = thread_shared_state.lock().unwrap();

            shared_state.has_completed = true;

            // The timer has completed, wake up the last task on which
            // the future was polled.
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}
