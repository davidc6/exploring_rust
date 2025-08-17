# Concurrency

- [Primitives](#primitives)
- [Memory Ordering](#memory-ordering)
- [Thread-local Storage](#thread-local-storage-tls)
- [Shared mutable state](#shared-mutable-state)
    - [Mutex](#mutex)
    - [Arc](#arc)

## Send and Sync

- A type is `Send` if it can be sent / moved (i.e. its' ownership can be transferred) 
to another thread.
- A type is `Sync` if it can be shared with another thread. The type is `Sync` if 
and only if it's a shared reference is `Send`. 

Raw pointers are not Sync or Send since the compiler does not know much about these. 
A way to implement these for the type:

```rs
struct SomeStruct {
    p: *mut i32
}

unsafe impl Send for X {}
unsafe impl Sync for X {}
```

Generally, all primitive types (i32, bool, str) are Send and Sync. These are auto 
traits. Auto traits get implemented for a type if its' fields are Send and Sync. 
If these traits are not needed, by implementing std::marker::PhantomData<T> marker 
trait, one can opt out of either.

### Which types are not Send and/or Sync then?

- Raw pointers (no Sync and Send) - since these have no safety guards. Dereferencing 
pointers is considered unsafe. 
- `Rc` (no Sync and Send) - reference (ref) count is shared and unsynchronised. 
If multiple threads held an Rc to the same allocation, there is a chance that 
they might try to modify the reference counter concurrency which will give unexpected 
results. (tip, Arc can come in handy in such cases). 
- `UnsafeCell` - `Cell` and `RefCell` are not Sync and Send either. UnsafeCell, 
on it's own, does not perform any synchronisation. UnsafeCell does not come with 
any conditions to avoid undefined behaviour. Usually it is not used on it's own 
but rather wrapped in any type that provides safety such as Cell or Mutex.

## Primitives

### Atomics

An atomic operation is an operation that cannot be cut into smaller pieces. This 
means that an operation is either fully completed or not (never happened). Multiple 
threads can atomically update a variable without causing undefined behaviour and 
so such operations are finished to completion before another thread attempts to 
carry out another operation on it.

All concurrency primitives (e.g. Mutex, CondVar) are built on top of atomic operations. 
Atomics are the building blocks of multithreading.

Rust has standard atomic types which have atomic methods. The availability of these 
depends on the system hardware and architecture. These reside under *std::sync::atomic*.

Modifications to atomics are possible due to interior mutability through a shared 
reference.

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

#### Basic Atomic Operations

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

#### Fetch-and-Modify

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




- Mutex 
- Condition variables

```rs use std::sync::atomic; ```

Availability of Atomic types depends on hardware architecture and OS. Most OSes
provide atomic types up to a pointer size (size depends on system architecture).

Modifications to Atomics are possible through a shared reference.

#### Memory ordering (Memory consistency)

Atomic operations take `Ordering` as an argument which determines the guarantees 
about the relative order of operations. 

- `Relaxed` is the simplest and weakest variant of memory ordering with fewest guarantees. It only guarantees that the access is atomic. Relaxed ordering gives no guarantees about the relative ordering of memory access across different threads. For example, two threads might see operations on different variables happen in a different order (say one thread write to variable a then b but another thread sees in in reverse order).
- Release and acquire ordering `Release`, `Acquire` and `AcqRel`.
    - Release and Acquire are used in a pair to form a happens-before relationship between threads
    - Release happens to store operations
    - Acquire happens to load operations
- Sequentially consistent ordering `SeqCst`.

The memory model defines the order in which operations happens in the happens-before relationships. The abstract model is a way to decouple from processor architectures. Only situations where something is guaranteed to happen before something else. The rule is that everything that happens within the same thread happens in order.

Mutexes and Semaphores are designed to care of memory reordering problems.

At the CPU level, memory instructions come in two main shapes: loads and stores. A load pulls bytes from a memory location into a CPU register. A store, stores bytes from a CPU register into a location in memory. These instructions usually operate on 8 bytes (chunks of memory) or less on modern CPUs. 

Two threads could potentially see operations on different variables in a
different order.

For instance, one thread writes to one the another variable, a different thread
might see this operation order in reverse.

## Thread-local storage (TLS)

A memory management method that uses static or global memory local to a thread.
Every thread as its own local copy of the variable. Unlike global variables, TLS
are only visible to the local thread.

https://learn.microsoft.com/en-us/cpp/c-language/thread-local-storage?view=msvc-170

There are several ways to use concurrency. For example:

- Fork-join parallelism (TODO)
    - These are deterministic and can't deadlock
- Channels (thread-safe one-way communication, one thread receiving the other sending)
- Shared mutable state (when using shared memory with synchronisation primitives)

## Fork-join parallelism

Diving the task into smaller tasks so that these can be executed in parallel, and joined afterwards.

## Channels

A channel is a mechanism to exchange and synchronize data between concurrently executing units of code. It's a conduit for one-way communication between threads (sending values from one thread to another). It can be thought of as a thread-safe queue. 

Unix pipes (an IPC mechanism) have a similar design where one entity sends data and other receives, typically from two different threads. In Unix, a pipe is a memory object. Rust channels however send Rust values (instead of bytes) where `send(value)` puts a value into a channel and `recv()` removes one. This follows the Rust ownership model where, the ownership of the value is transferred from the sending thread to the receiving one.

This method of communication allows threads to communicate directly without locking or shared memory. This concurrency method has been used in Erlang for many years. 

Rust's channel are multi producer, single consumer (mpsc). This is a message passing communication pattern (CSP - communicating sequential processes).

```rs
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};

struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

// heap-allocated, atomically reference counted
// | 
// V   
// Arc<Mutex<Receiver<SomeEvent>>>
let (sender, receiver) = channel(); 
let value = SharedReceiver(Arc::new(Mutex::new(receiver)));
```

## Shared mutable state

When access is needed by many threads to a single (sharable) location. It should mutable an accessible by all threads. Below, we discuss critical section which is a segment of code that accesses a shared resource. This code must be executed only by one thread or process at a time. Implementing a critical section in a concurrent program, can prevent corruption, race conditions and deadlock.

### Mutex

- These support programming with invariants (never-changing rules that set up at the construction and maintained by every critical section). Invariants are properties of code that remain true throughout its (program's) execution or specific context (i.e. class or loop). 

```rs
use std::sync::{Mutex, Arc};

type UserId = u32;
type WaitingList = Vec<UserId>;
const CHAT_SIZE: usize = 8;

/// All threads can access this
struct ChatApp {
    waiting_list: Mutex<WaitingList>
}

let app = Arc::new(ChatApp {
    waiting_list: Mutex::new(vec![])
});

impl App {
    fn join_waiting_list(&self, user: UserId) {
        // Mutex dynamically enforces exclusive access.
        // This is usually done statically, at compile time by the compiler.
        // This is interior mutability, similar to RefCell but with support for multiple threads.
        let mut guard = self.waiting_list.lock().unwrap();

        guard.push(user);

        if guard.len() == CHAT_SIZE {
            let users = guard.split_off(0);
            self.start_chat(users);
        }

        // When the guard is dropped, the lock is released.
        // Can also be manually triggered i.e. drop()
    }
}
```

#### Challenges with Mutexes

- Safe Rust cannot cause data race (when threads read and write to the same memory concurrently resulting in unexpected results).
- Other types of race conditions can be triggered by timing of threads thus program behaviour might vary.
- Mutexes can lead to monolithic code if not managed correctly.
- Deadlock
    - A thread can deadlock itself by trying to acquire a lock that's holding
        - Suppose thread one acquires the lock on the first call. Then on the second call it sees that the lock is held so it blocks. The thread blocks and it will never get released since the thread that acquired it, is the one holding it.
    - Multiple threads that each acquire multiple mutexes at once
    - Two threads might block while waiting to receive a message from each other. This can happen with channels.

```rs
// Take the lock
let guard = self.waiting_lock.lock().unwrap()
// Block and wait for the lock to get released
let guard = self.waiting_lock.lock().unwrap()
```

#### Poisoned Mutex

If a thread panics with holding a Mutex, it's marked as poisoned. If programming with invariants (a condition that is always expected to be true) then the concern is that it's possible that the invariants are broken. Perhaps some data was updated but not the other and the program panicked and bailed out of the critical section without completing what it was doing. Rust poisons the Mutex to prevent other threads from operating on a potentially broken data. Poisoned mutex still can be locked and inner data accessed if necessary.

#### Resources

- [Fair Mutex in parking lot](https://docs.rs/parking_lot/latest/parking_lot/type.Mutex.html)
- [Mutex Flavors](https://www.intel.com/content/www/us/en/docs/onetbb/developer-guide-api-reference/2021-6/mutex-flavors.html)

### Read/Write Locks (RwLock<T>)

If most the operations that threads do are read-heavy. Mutex can be used but there is not need for threads for wait just to execute a read operation. This is a good use-case for RwLock. Essentially it's a multi-threaded version of RefCell but it does not panic on conflicting borrows.

RwLock has two locking methods (read and write). `read` provides exclusive mutable access to the data, `read` on the other hand non-mutable.

#### Challenges with RwLock

It can lead to writer starvation if too many readers/reads. However, the implementation is dependent on the system and most will prevent new reads if a writer is waiting.

#### Summary

Both Mutex<T> and RwLock<T> require `T` to be `Send` since they can send `T` to another thread. `RwLock<T>` also needs to be Sync since multiple threads can holds a shared reference to `T`. 

### Condvar (Condition Variables)

These are often used for waiting for something to happen to data that is protected by a mutex. There are two operations that condvars support `wait` and `notify`. Threads can wait for a condition variable and they can be woken up when another thread notifies the same condvar. Multiple threads can wait for a condvar and either one or all can be notified. 

Conditional variable can be created for a specific events or conditions. For example, wait until the queue is empty and when it does become empty (say a thread emptied a queue) then notifies the conditional variable which in turn notifies all the threads that are waiting on it. 

Condvars provide a mechanism to atomically unlock the mutex and start waiting so there's no way to miss the notification. Condvar only works with a Mutex. 

### Arc

Arc (stands for "Atomically Reference Counted") - allows shared ownership through reference counting. It is a smart pointer.

```rs
// Creates a new allocation.
let arc = Arc::new();

// Produces a new Arc instance that points to the same allocation on the heap (as the source).
let arc_two = arc.clone();
```

Shared references disallow mutation by default. In order to mutate:

1. Interior mutability using Mutex, RwLock or one of the Atomics (
2. COW (copy-on-write) semantics `Arc::make_mut` which clones the data only when needed (if there references to it) without using interior mutability.
3. `Arc::get_mut` when the reference count is 1 for the direct mutable access.

```rs
use std::sync::Arc;

fn main() {
    let mut data = Arc::new(vec![1, 2, 3]);
    // Cloning Arc increments reference count.
    let mut data_two = data.clone();
    // Makes a copy of the "data" since there is a reference to it.
    Arc::make_mut(&mut data).push(4);
   
    // Prints: [1, 2, 3, 4]
    println!("{:?}", *data);
    // Prints: [1, 2, 3]
    println!("{:?}", *data_two);
}
```

#### Memory layout

`Arc<T>` where `T` is the value allocated on the heap. `Arc` need to think about synchronisation since the reference count is shared mutable state. Since using something like `Mutex` is an overkill, atomics can be used instead. Since there's a pointer to the T's allocation, extra metadata (in this case reference count) can live there too. 

- If `T` is sized then on the stack there is a single (thin) pointer pointing to the heap allocation of T as well as weak and strong ref counts.
- If `T` is not sized then there is an extra bit of information required to be stored on the stack about dynamically sized T. This can be length of the slice or vtable pointer.

#### Weak<T>

A `Weak<T>` is also called a weak pointer (which we touched on earlier), does not prevent an object from getting dropped, meaning that when all Arc<T>'s are dropped, T is dropped too regardless of the existence of any `Weak<T>`s. T can be shared between Arc and Weak. 

In structures when Arc's is used extensively, Weak can be used for example to break structures in a child-parent relationship where child uses weak for their parent node and dropping parent is not prevented that way. 

#### Demo Implementation

##### Basic Reference Counting

```rs
use std::sync::atomic::{AtomicUsize, fence};
use std::sync::atomic::Ordering::{Relaxed, Acquire, Release};
use std::ptr::NonNull;
use std::ops::Deref;

// Data stored in Arc
struct Data<T> {
    /// Reference count
    ref_count: AtomicUsize,
    /// The data
    data: T,
}

// Actual interface
pub struct Arc<T> {
    /// The pointer has to be guaranteed not to be null.
    ptr: NonNull<Data<T>>
}

impl<T> Arc<T> {
    /// Box::new - creates new allocation on the heap.
    /// 
    /// Box::leak - used here to get access to the value on the heap 
    /// and return a mut reference to it &'static mut, giving up
    /// exclusive ownership of this allocation.
    ///
    /// This way we are preventing deallocation by Box.
    /// This is similar to leaking memory in other languages
    /// but we are doing it on purpose here.
    ///
    /// NonNull turns it into a pointer
    fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(Data {
                ref_count: AtomicUsize::new(1),
                data
            })))
        }
    }
   
    /// Wrapper since we need unsafe code to access the raw pointer.
    fn data(&self) -> &Data<T> {
        unsafe { self.ptr.as_ref() }
    }
}

// Deref is used here for immutable dereferencing (i.e. *value).
// No DerefMut implementation since no mutable ownership is allowed.
impl<T> Deref for Arc<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.data().data
    }
}

// Clone implementation increments ref counter and returns the same pointer.
// Relaxed ordering is used here since no memory other than the one being operated on is touched.
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // To reduce the possibility of other threads calling Arc::clone while the process is being aborted,
        // we set the limit to half of usize MAX.
        if self.data().ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc {
            ptr: self.ptr
        }
    }
}

/// Using "Release" memory ordering here since it's relevant for write/store operations.
/// To make sure that nothing is still accessing the data after final drop.
///
/// The last/final fetch_sub establishes a happens-before relationship with all previous 
/// operations. We can do this using release/acquire ordering. 
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Release) == 1 {
            // If last, acquire data ownership.
            // 
            // Make previous non-Acquire atomic loads into Acquire
            // if they have a matching Release atomic store.
            fence(Acquire);

            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}
```

#### Resources

- [Leaking memory on purpose in Rust](https://softwaremill.com/leaking-memory-on-purpose-in-rust/)
- [Layout](https://softwaremill.com/leaking-memory-on-purpose-in-rust/)
- [Chapter 6. Building Our Own "Arc"](https://marabos.nl/atomics/building-arc.html)

## AI-generated

### A list of common concurrency models

| Model                 | Key Idea                       | Shared Memory? | Examples                   |
| --------------------- | ------------------------------ | -------------- | -------------------------- |
| Thread-based          | OS threads, shared state       | Y              | Rust threads, Java threads |
| Actor                 | Message passing between actors | N              | Erlang, Actix              |
| Async/Await (Futures) | Non-blocking tasks             | N / Optional   | Rust async, JS, Python     |
| Shared Memory + Locks | Explicit synchronization       | Y              | Mutex, RwLock, Atomics     |
| CSP (Channels)        | Message passing via channels   | N              | Go, Rust channels          |
| STM                   | Memory transactions            | Y              | Haskell STM, Clojure refs  |
| Fork/Join             | Parallel tasks                 | Maybe          | Rayon, Java ForkJoin       |
| Green Threads         | User-space threads             | Depends        | Go, Tokio                  |
| Dataflow/Reactive     | Triggered by data change       | N              | RxJava, Streams            |

## TODO

- How's Mutex implemented under the hood?
