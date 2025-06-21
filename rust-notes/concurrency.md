# Concurrency

- [Primitives](#primitives)
- [Memory Ordering](#memory-ordering)
- [Thread-local Storage](#thread-local-storage-tls)

## Send and Sync

A type is `Send` if it can be sent (i.e. its' ownership can be transferred) to another thread.

A type is `Sync` if it can be shared with another thread. The type is `Sync` if and only if it's a shared reference is `Send`. 

Raw pointers are not Sync or Send since the compiler does not know much about these. A way to implement these for the type:

```rs
struct SomeStruct {
    p: *mut i32
}

unsafe impl Send for X {}
unsafe impl Sync for X {}
```

## Primitives

- Atomics
- Mutex 
- Condition variables

```rs use std::sync::atomic; ```

Availability of Atomic types depends on hardware architecture and OS. Most OSes
provide atomic types up to a pointer size (size depends on system architecture).

Modifications to Atomics are possible through a shared reference.

## Memory ordering

Two threads could potentially see operations on different variables in a
different order.

For instance, one thread writes to one the another variable, a different thread
might see this operation order in reverse.

`Relaxed` is the simplest variant of memory ordering with fewest guarantees.

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

Rust's channel are multi producer, single consumer (mpsc). This is a message passing communication pattern (CSP - communicating sequential processes).  

## Shared mutable state

When access is needed by many threads to a single (sharable) location. It should mutable an accessible by all threads.

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
- Deadlocks
    - A thread can deadlock itself by trying to acquire a lock that's holding
    - Multiple threads that each acquire multiple mutexes at once

```rs
// Take the lock
let guard = self.waiting_lock.lock().unwrap()
// Block and wait for the lock to get released
let guard = self.waiting_lock.lock().unwrap()
```

#### Poisoned Mutex

If a thread panics with holding a Mutex, it's marked as poisoned. If programming with invariants then the concern is that it's possible that the invariants are broken. Perhaps some data was updated but not the other and the program paniced and bailed out of the critical section without completing what it was doing. Rust poisons the Mutex to prevent other threads from operating on a potentially broken data. Poisoned mutex still can be locked and inner data viewed if necessary.

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

Arc (stands for "Atomically Reference Counted") - allows shared ownership through reference counting. 

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


## Resources

- TODO

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
