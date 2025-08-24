# Memory Ordering

Compilers can performs many different optimisations to make programs run faster. 
For instance, if consecutive instructions can be re-ordered and executed out of 
order without affecting the outcome then they will be re-ordered. As long as these 
optimisations do not change the behaviour of the program, they will be applied to 
make the program run faster.

Memory ordering is defined in terms of an abstract memory model.

## Memory Consistency



## Memory Model

A programming language memory model is what mechanisms can parallel programs rely 
on to share memory between threads. 

Rust's memory model is an abstract model with a strict set of rules 

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
