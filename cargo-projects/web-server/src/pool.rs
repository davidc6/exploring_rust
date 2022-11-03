use std::{thread, sync::{mpsc, Arc, Mutex}};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // JoinHandle - enables associated thread blocking
        let thread = thread::spawn(move || loop {
            // acquire mutex and block current thread, wait for value on the receiver and panix if any errors
            // acquiring a lock might fail if mutex is in a poised state i.e. other thread paniced while holding the lock
            match receiver.lock().unwrap().recv() {
                // we successfully acquired the lock
                Ok(job) => {
                    println!("Worker {id} got a job, executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }

        });

        Worker { id, thread: Some(thread) }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

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
}