# Concurrent programming

## Synchronization

Synchronization one of the fundamental concepts in concurrent programming. It is a way of coordinating access to shared data by multiple threads. Concurrent modules that share memory must be able to synchronize with each other in order to yield correct outcomes.

Lock is a synchronization technique that allows only one thread to take ownership of the shared data. When a thread is holding a lock (the lock is the "acquired" state) it tells that other thread(s) that it is mutating data and no one thread can touch it. Once the thread is done it releases the lock (released state) and other thread(s) can acquire it.

## Multithreading

- Why threads?

- In real-world applications a program may use a thread for keyboard input, another for the screen updates and a number of other threads for other things.

### Options to improve throughput of a web server

- Fork/Join model

Execution branches off ("fork") in parallel at certain points to join/merge ("join") at a certain point and resume sequential execution. Typically a threadpool is used to execute forked tasks, fibers or non-OS/lightweight threads. This design helps with the overhead of creating new, running and terminating threads. Tasks are a lot like threads but much more lightweight. Each threadpool has a fixed number of threads and each thread has a queue of tasks that are awaiting to get executed. When a new task is forked it is added to the queue of the thread that is executing its parent task. 

- The single-threaded async (non-blocking) I/O model

Typically when a single thread is used to handle concurrent tasks (asynchronously) rely on a concept called "event loop". An event loop is a continuous loop that checks for events. If watches a queue of tasks. A task generates events (such as an I/O operation completion), the event loop processes the task. An OS kernel provides system calls that notify of I/O events (such as `epoll` on Linux, `kqueue` on MacOS). 

- The multi-threaded async I/O model

Let's say if a thread waits too long for data to return from an IO source (network, hard drive etc.) most likely it will be interrupted by a (pre-emptive) thread scheduler. This allows a different thread to become active. 

Pre-emptied - 

Blocked by I/O - a thread has to wait for data to return from an IO source (network, hard drive etc.)

- Threadpool

### Info

- If a parent thread terminates then child thread(s) terminate(s) also
- Thread management operations
    - Thread creation (self explanatory)
        - Parent thread is the main thread and a thread that gets created from this thread is child thread.
    - Thread termination
        - Threads terminate at some point once the assigned task completes. Once the parent thread terminates, all of it's child threads terminate also (unless join operation is used)
    - Thread join
        - This operation can be executed to wait until the other thread terminates, in general a parent thread joins a child thread. If parent thread joins child thread and it is still running, then parent thread is suspended until child thread terminates. If parent thread takes longer than child thread to execute then join has no effect. Parent thread can join many child threads or some child threads. Those child threads that are not joined by the parent thread, will terminate once parent thread gets terminated.
    - Thread yield
        - It is important to control the thread so that it does not take over the CPU cycles and hogs it, therefore it is important that is releases itself. Threads should take a break to allow other threads to operate. A thread yield allows a thread to get suspended (ready queue) and other runnable thread starts using the CPU.

Ref - https://pages.mtu.edu/~shene/NSF-3/e-Book/FUNDAMENTALS/thread-management.html
Ref - https://pages.mtu.edu/~shene/NSF-3/e-Book/FUNDAMENTALS/threads.html
Multithreading - https://pages.mtu.edu/~shene/NSF-3/e-Book/index.html

- Message-based communication

Multi producer - single consumer

                    <- Producer (Sender)
Consumer (Receiver) <- Producer (Sender)
                    <- Producer (Sender)

In Rust channels are multiple-producer, single-consumer channels

## Atomics

An operation that cannot be split up and has to either be complete fully or not happen at all.

Atomic operations are the building blocks when multiple threads are involved. Concurrency primitives such as mutexes are implemented using atomic operations.

## Locks

### Spinlock

- One of the simplest mutual exclusion lock mechanisms
- The idea is that the code keeps spinning/looping until is_locked == false is returned (until lock is acquired)
- Thread that keeps on trying to acquire the lock is put into a busy waiting/looping mode while waiting to acquire the lock
- Since the caller context hogs the CPU until the lock is acquired, it is important to not use within critical sections
- Modern spinlock code is usually nightly optimised and does not "literally spin" the CPU (spinlocks are meaningly on multi-core machines)
- Although this mechanism is a resourceful, it works well when locked briefly

- [Simple Spinlock implementation](./src/spin_lock.rs)

### Mutex

To share mutable data between threads, the most common tool is Mutex (mutual exclusion). Mutex enables only one thread to have exclusive access 
to the data. This is done by temporarily blocking access to the data for other threads. There are only two states "locked" and "unlocked".

When a thread tries to lock a mutex and it's unlocked, it immediately proceeds to locking it causing all other threads that come after it to sleep. Once the thread is done with it, it unlocks the mutex and then one of the awaiting threads gets woken up and tries to lock the mutex.

It essentially is an agreement between threads to access data one when mutex is locked.

#### Lock poisoning

When a thread that is holding the lock panics, a Mutex in Rust gets poisoned. When it gets poisoned, Mutex is no longer locked but locking it again will Err (because it has been poisoned). Since a thread that was operating on the data that Mutex holds panicked, data consistency can no holder be guaranteed. Usually, poison is disregarded or panic is propagated to the users of Mutex.

### Reader-Writer

This lock is split into reader and writer interfaces. RwLock allows multiple readers of the underlying data or a single (exclusive) writer to the underlying data. There are three states: unlocked, multiple readers lock or single writer lock. Most commonly this lock is used when data is often accessed but gets updated/written once in a while by multiple threads. RwLock can be thought of as a multithreaded version of RefCell.

It is worth noting that T needs to be Send to transfer data between threads. RwLock also required T to be Sync since multiple threads can hold a shared reference to T (&T).

## Interior mutability

Data types that allow interior mutability make it possible to mutate data through the normal immutable reference.

### Cell<T>

Wraps a T and allows mutations through a shared reference. Cell allows to either copy the value out or replace it with another value.

### RefCell<T>

Allows to borrow the content. Additionally, it holds a counter that keeps tracks of outstanding borrows. If borrowing while already mutably borrowed, it will error. RefCell can only be used on a single thread.

### Arc and Mutex

### Atomics

These are concurrent version of Cell. Values have to be copied without letting us borrow the contents directly. 

Atomics cannot be arbitrary size and are limited in scope. Given the limited nature of Atomics, they don't contain the information 
that needs to be shared between threads. Usually these are used a tool to share bigger things between threads. 

### UnsafeCell

All interior mutability types are built on top of UnsafeCell which is usually not used directly as it returns a raw pointer when it's get() method gets called. This can only be used in an unsafe block. 

## Send, Sync

Both are these are traits are auto traits and they are implemented automatically for types based on fields. 

If a type is Send then it can be safely sent (its ownership can be transferred) across to another thread. 
    - Arc is Send

If a type is Sync then it can be safely shared across threads.
    - A type is Sync if shared ref to that type is also Send e.g. i32 (all primitive types are) is Sync

### Resources

- [Threads](https://people.cs.rutgers.edu/~pxk/416/notes/05-threads.html)
- [Basic Thread Management](https://pages.mtu.edu/~shene/NSF-3/e-Book/FUNDAMENTALS/thread-management.html)A
- [Fork/join parallelism](https://ycpcs.github.io/cs365-spring2017/lectures/lecture13.html)
