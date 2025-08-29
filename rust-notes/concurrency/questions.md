# Questions

1. To achieve concurrency, what primitives would you use?

1) Threads - `std::thread` to spawn threads.
2) Message passing - `std::sync::mpsc` mechanism to communicate between threads. 
This allows to avoid shared memory communication since concurrency happens through 
message passing.
3) Shared memory with synchronisation primitives
    a) Arc (thread-safe reference-counted) implements Send and Sync
    a) Mutex (mutual exclusion) implements Sync (but not Send), the guard (returned 
    by the `.lock()` implements Sync (not Send) since Mutex must be unlocked by the 
    same thread that locked it. 

    In order for Mutex to be Send, T needs to be Send. T must be Send for Mutex 
    to be Sync.

    c) RwLock (read-write lock)

    d) Atomic types for lock free programming

    e) 

Lock-based
    1) Mutex
    2) RwLock

Lock-free
    1) Atomics

2. What primitives does the OS give you?

TODO

3. What is the difference between threads and async programming?

Async programming can be done using a single thread. A thread is an OS primitive 
that enables to execute code concurrently on multiple threads. Async programming is 
a paradigm of writing concurrent applications. Both are used to achieve concurrency. 

Threads a good for CPU-bound whereas async IO-bound tasks.
