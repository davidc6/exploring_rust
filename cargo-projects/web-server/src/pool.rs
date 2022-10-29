use std::{thread, sync::{mpsc, Arc, Mutex}};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // JoinHandle - enables associated thread blocking
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

        ThreadPool { workers, sender }
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
        println!("inside execute");
    }

    // implement the build function
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}
}
