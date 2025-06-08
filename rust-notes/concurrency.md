# Concurrency

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

### 


## TODO

- How's Mutex implemented under the hood?
