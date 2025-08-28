# Memory Ordering

Memory ordering is sometimes also called memory consistency. Consistency is about 
(global) ordering of all memory operations from different processors (threads, processors) 
to different memory locations. It can be thought of as a contract between the 
application programmer and the system. When a shared memory location is modified 
by a processor, when does is it made visible to other processors that have it in their 
private caches. What values can loads return based on memory ordering, fences and 
other processes that take place during the execution on a program. 

Defines the order of reads and writes to memory locations.

Compilers can performs various optimisations to make programs run faster. 
For instance, if consecutive instructions can be re-ordered and executed out of 
order without affecting the outcome, then they will be re-ordered. As long as these 
optimisations do not change the behaviour of the program, they will be applied to 
make the program run faster. This happens in absence of any atomics and other 
synchronisation primitives. This is called **as-if rule** and can cause issues in 
multithreaded environments. 

Memory ordering is defined in terms of an abstract memory model. It is the order 
in which memory is accessed. 

Memory consistency is about the order of operations, behaviour of reads and writes 
to different locations (as observed by processors). It is about when writes to 
some address propagate to other processors relative to reads and writes to other 
addresses.

It defines the allowed load and store behaviours to different addresses in a 
parallel system.

To put it simply if there are operations A, B, C and D, in what order should these 
operations be executed at a hardware level? There should be a contract between 
the programmer and micro architect (ISA specification). This helps with debugging,
ease of state recovery and exception handling. 

## Memory ordering in a Single Processor

- von Neumann model / architecture (sequential execution)
- Hardware executes the load and store operations in order specified

## Memory Model

Why do we need a memory model?

There needs to be a spec, a set of rules around what can be cannot be returned.

There is a wide range of memory models. For example:

1) Sequential (consistency) memory model is hard to implement for high performance.
2) Total Store Ordering (TSO) - 
3) RISC-V (RVWMO) - 
3) IBM Power / Nvidia GPUs - 

A programming language memory model is about how things are laid out in memory, 
as well as concurrency aspects.

Say we have a variable and two threads. The first thread reads the variable and
is guaranteed to observe writes to the same variable by the second thread. A memory 
model specifies the semantics under which this happens.

Rust's memory model is an abstract model with a strict set of rules. It is a lot 
like C++ memory model. 

Memory models is also about specifying the mechanisms a programs can rely on to 
share memory between threads. The memory model defines the order in which operations 
happen (in terms of happens-before relationships). 

Within the same thread, everything happens in order. Between threads, happens-before 
occurs when 

a) spawning and joining a thread
b) locking and unlocking a mutex
c) working with non-relaxed atomic operations.

*Relaxed* memory ordering is the most performant and basic one. It does not result 
in any cross-thread happens-before relationships. This memory model has been quite 
popular in the recent years and allows for many compiler and CPU optimisations.

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

## Relaxed Ordering


