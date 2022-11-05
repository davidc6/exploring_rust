use std::{thread, sync::{mpsc, Arc, Mutex}};

// Worker struct is used to manage behaviour between
// the thread and Threadpool
// It picks up the code that needs to be run in the threads and runs it
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // JoinHandle - enables associated thread blocking
        // move enables closure to own variables it uses
        let thread = thread::spawn(move || loop {
            // any temp values (in our case the lock) on the right hand side are dropped when let statement ends
            // if expression is used directly after match, it is not dropped until the end of the assosiated block
            // meaning that the lock will be held for the duration of the call to job()
            let message = receiver.lock().unwrap().recv();
            // acquire mutex and block current thread, wait for value on the receiver and panic if any errors
            // acquiring a lock might fail if mutex is in a poised state i.e. other thread paniced while holding the lock
            match message {
                // we successfully acquired the lock
                Ok(job) => {
                    println!("Worker {id} got a job, executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // number of threads should be positive
        assert!(size > 0 && size < 11);

        // using message passing to transfer data between threads
        let (sender, receiver) = mpsc::channel();
        // receiver gets wrapped in 
        let receiver = Arc::new(Mutex::new(receiver));

        // preallocate space in vector
        let mut workers = Vec::with_capacity(size);

        // we create a range from 0 to whatever the size of the threadpool is
        // we share the receiver by cloning its pointer
        // When Arc (Atomic Reference Counted) gets cloned, a new instance is created and reference count is increased
        // This gives workers the ability to share ownership of the receiver 
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Threadpool creates a channel, holds on to the sender and each worker to the receiver
        // each worker will hold closures we want to send down the channel
        // worker's execute method will send the job through the sender
        // worker will loop over it's receiver and execute the closures
        ThreadPool { workers, sender: Some(sender) }
    }

    // F - generic type which is bound by FnOnce(), Send and lifetime 'static
    // FnOnce() represents a closure that takes no params and returns a unit type and will only execute once
    // Send trait is for types that can be transfered across thread boundaries
    // 'static -  
    // where - a clause that specifies constraints on lifetime and generic parameters
    // Send - types that implement this trait if it is safe to share data between threads
    // 'static - lifetime indicates that the value has to life for the entire lifetime of the program
    // in this context we don't know how long it will take the thread to execute
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // creating a job instance
        // putting it on the heap
        let job = Box::new(f);
        // since sender is an Option that owns its content (some job)
        // as_ref() converts the option to not owning its content
        // i.e. &Option<Sender<..>> to Option<&Sender<..>>
        // in other words, as_ref() does not consume the data but borrows it (Sender)
        // then send it down the channel
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// implements drop trait
impl Drop for ThreadPool {
    // when the threadpool goes out of scope each thread joins
    fn drop(&mut self) {
        // drops the sender before thread finishes
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use super::{ThreadPool};

    #[test]
    fn threadpool_instantiates_with_4_threads() {
        let tp = ThreadPool::new(4);
        assert_eq!(4, tp.workers.len());
    }

    #[test]
    #[should_panic]
    fn threadpool_panics_if_zero() {
        ThreadPool::new(0);
    }

    #[test]
    #[should_panic]
    fn threadpool_panics_if_above_ten() {
        ThreadPool::new(11);
    }

    #[test]
    fn threadpool_executes() {
        const THREAD_COUNT: usize = 2;
        let tp = ThreadPool::new(THREAD_COUNT);

        // trasmitter and receiver tuple returned by channel
        // get bound to tx and rx
        let (tx, rx) = channel();

        // iterate THREAD_COUNT times, execute fn by sending data down the channel
        for _ in 0..THREAD_COUNT {
            // clone sender/transmitter to send to other threads
            let tx = tx.clone();
            // transmit / send value to the channel
            tp.execute(move || {
                tx.send(1).unwrap();
            })
        }

        // receiver returns an iterator, takes the number of threads and sums the items
        // i.e. 1 + 1 (if THREAD_COUNT is 2)
        assert_eq!(rx.iter().take(THREAD_COUNT).sum::<usize>(), THREAD_COUNT);
    }
}
