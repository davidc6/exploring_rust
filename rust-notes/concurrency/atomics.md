# Atomics

An atomic operation is an operation that cannot be cut into smaller pieces. This 
means that an operation is either fully completed or not (never happened). Multiple 
threads can atomically update a variable without causing undefined behaviour and 
so such operations are finished to completion before another thread attempts to 
carry out another operation on it.

Briefly, Atomics work by using special CPU instructions to perform memory operations as a 
single, uninterruptible step making sure that it happens without any interference 
from other threads.

All concurrency primitives (e.g. Mutex, CondVar) are built on top of atomic operations. 
Atomics are the building blocks of multithreading.

Rust has standard atomic types which have atomic methods. The availability of these 
depends on the system hardware and architecture. These reside under *std::sync::atomic*.

Modifications to atomics are possible due to interior mutability through a shared 
reference.

Here's a simple example to demonstrate how atomics work:

```rs
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let value = AtomicUsize::new(1);
    // Ordering is required to let the compiler now how to synchronise memory.
    // SeqCst guarantees sequential consistency.
    // prev_value will be set to the previous AtomicUsize value (i.e. 1)
    let prev_value = value.fetch_add(1, Ordering::SeqCst);
}
```

For example, **AtomicUsize** has methods such as **fetch_add()** which adds to the 
current value returning the previous value. Each available atomic type has the 
same API for storing and loading (fetch-and-modify) operations. 

You'll notice in the example above that we are passing in *Ordering* as the second 
argument o *fetch_add()* method. **Memory Ordering** is a concept that allows us to 
define what guarantees we get about the relative order of operations. 

For instance, if there are threads and the first one writes to variable A then to 
variable B (in this particular order), thread 2 might see it in an opposite order. 

## Basic Atomic Operations

```rs
impl AtomicUsize {
    pub fn load(&self, ordering: Ordering) -> usize;
    pub fn store(&self, value: usize, ordering: Ordering);
}
```

- *load()* method loads the value stored in the atomic variable
- *store()* method stores the value (atomically) in the atomic variable

Here's an example to demonstrate simple load and store operations example.

```rs
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    static COUNTER: AtomicUsize  = AtomicUsize::new(0);
    
    // Spawned thread
    let bg_thread = thread::spawn(|| {
        // Check the current counter, break out of loop if the condition 
        // is satisfied or continue the loop otherwise
        loop {
            let current = COUNTER.load(Ordering::Relaxed);
            
            if current == 1 {
                break;
            }

            // do some work here
        }
    });
   
    // Main thread
    COUNTER.store(1, Ordering::Relaxed);
    
    let _ = bg_thread.join();
}
```

Another example where the value is being updated and read in real-time.


```rs
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn some_task(_value: usize) {
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let counter = AtomicUsize::new(0);

    let main_thread = thread::current();
    
    // Scoped threads here, enable auto join handling and local variable borrowing.
    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                some_task(i);
                counter.store(i + 1, Ordering::Relaxed);
                // This will wake up the main thread.
                main_thread.unpark();
            }
        });
        
        loop {
            let number_of_done_tasks = counter.load(Ordering::Relaxed);
            println!("Tasks done: {number_of_done_tasks}");
            if number_of_done_tasks == 100 {
                break;
            }
            // Blocks until current thread's token is available or Duration limit
            // is reached (3s in this case).
            thread::park_timeout(Duration::from_secs(3));
        }
    });
    
    println!("Operation completed!");
}
```

## Fetch-and-Modify

- *fetch_add(&self, value: usize, ordering: Ordering)* - addition operation, returns
the old value and wraps around if a value is past the maximum representable value.
This is quite different to the standard "+" and "-" operations or integers.

We can redo our counter example using this method. Since the order in which the
threads will increment the counter is not known, we can rely on the atomic operations 
to be exactly sure that it will be a 100 in the end when all threads are done.

```rs
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn some_task(_value: usize) {
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let counter = &AtomicUsize::new(0);
    
    // Scoped threads here, enable auto join handling and local variable borrowing.
    // Counter is now a reference.
    // Closures capture "t" since capturing by reference isn't allowed.
    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    some_task(t * 25 + i);
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
        
        loop {
            let number_of_done_tasks = counter.load(Ordering::Relaxed);
            println!("Tasks done: {number_of_done_tasks}");
            if number_of_done_tasks == 100 {
                break;
            }
            // Blocks until current thread's token is available or Duration limit
            // is reached (3s in this case).
            thread::park_timeout(Duration::from_secs(3));
        }
    });
    
    println!("Operation completed!");
}
```

An example of using several atomics.

```rs
use std::sync::atomic::{AtomicUsize, Ordering::{Relaxed}};
use std::thread;
use std::time::{Duration, Instant};

fn do_some_work(i: usize) {
   println!("Doing busy work...");
}

fn main() {
    let counter = &AtomicUsize::new(0);
    let time_taken = &AtomicUsize::new(0);
    let time_max = &AtomicUsize::new(0);

    thread::scope(|scope| {
        for t in 0..4 {
            scope.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    do_some_work(t * 25 + i);
                    let elapsed = start.elapsed().as_micros() as usize;
                    counter.fetch_add(1, Relaxed);
                    time_taken.fetch_add(elapsed, Relaxed);
                    time_max.fetch_max(elapsed, Relaxed);
                }
            });
        }
            
        loop {
            let tt = Duration::from_micros(time_taken.load(Relaxed) as u64);
            let tm = Duration::from_micros(time_max.load(Relaxed) as u64);
            let c = counter.load(Relaxed);

            if c == 100 {
                break;
            }

            if c == 0 {
                println!("Have not started yet.");
            } else {
                println!("Tasks completed {c} out of 100, max {:?}, avg {:?}", tm, tt / c as u32);
            }
        }
    });

    println!("Operation completed");
}
```


- Mutex 
- Condition variables

```rs use std::sync::atomic; ```

Availability of Atomic types depends on hardware architecture and OS. Most OSes
provide atomic types up to a pointer size (size depends on system architecture).

#### Memory ordering (Memory consistency)

Atomic operations take `Ordering` as an argument which determines the guarantees 
about the relative order of operations. 

- `Relaxed` is the simplest and weakest variant of memory ordering with fewest 
guarantees. It only guarantees that the access is atomic. Relaxed ordering gives 
no guarantees about the relative ordering of memory access across different threads. 
For example, two threads might see operations on different variables happen in a 
different order (say one thread write to variable a then b but another thread sees 
in reverse order).

- Release and acquire ordering `Release`, `Acquire` and `AcqRel`.
    - Release and Acquire are used in a pair to form a happens-before relationship 
    between threads
    - Release happens to store operations
    - Acquire happens to load operations
- Sequentially consistent ordering `SeqCst`.

The memory model defines the order in which operations happens in the happens-before 
relationships. The abstract model is a way to decouple from processor architectures. 
Only situations where something is guaranteed to happen before something else. 
The rule is that everything that happens within the same thread happens in order.

Mutexes and Semaphores are designed to care of memory reordering problems.

At the CPU level, memory instructions come in two main shapes: loads and stores. 
A load pulls bytes from a memory location into a CPU register. A store, stores 
bytes from a CPU register into a location in memory. These instructions usually 
operate on 8 bytes (chunks of memory) or less on modern CPUs. 

Two threads could potentially see operations on different variables in a
different order.

For instance, one thread writes to one the another variable, a different thread
might see this operation order in reverse.

