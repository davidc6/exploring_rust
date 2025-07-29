# Low Latency Rust Techniques

Traditional synchronization mechanisms can introduce:

- Contention (in multi-threaded code, contention is when two or more threads are 
attempting to access the same resource simultaneously)
- Deadlocks (when two or more processes are blocked indefinitely, each waiting 
for a resource that the other holds).
- Priority inversion (when a high-priority thread is indefinitely superseded by 
a lower-priority thread)

All these things can lead to increased latency. Lock-free data structures can 
significantly reduce latency. If programming with multiple threads that access 
shared memory and these thread cannot block each other then it's "lock-free 
programming". Lock free programming essentially describes the fact that the
possibility of locking up the entire application is (deadlock, livelock, thread
scheduling decisions) is very low if not impossible at all. Large applications 
do not completely consist of lock-free operations but usually a specific set of
operations.

If a single thread is suspended, it will not prevent other threads from making 
progress. This is really valuable for real-time systems where tasks have to 
complete within a certain time limit.

- Atomic operations
- Memory barriers
- ABA problem 

- Thread-safe access to shared data without the use of synchronization primitives 
such as mutexes
- Hardware support for lock-free programming is important but can also be done 
without it, however it's not very practical
- Designing generalised lock-free algorithms is hard
- Lock-free data structures design is more practical (such as Buffer, List, Queue, 
Map etc)
- Lock-free stack is an example

