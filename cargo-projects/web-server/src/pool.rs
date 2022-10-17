use std::{thread, sync::{mpsc, Arc, Mutex}};


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        
        Worker { id, thread }
    }
}

struct Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     if size < 1 {
    //         PoolCreationError
    //     }

    //     Ok(ThreadPool)
    // }

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // preallocate space in vector
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // F - generic type which is bound by FnOnce(), Send and 'static
    // FnOnce() represents a closure that takes no params and returns a unit type
    // Send trait is for types that can be transfered across thread boundaries
    // 'static -  
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {}

    // implement the build function
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}
}