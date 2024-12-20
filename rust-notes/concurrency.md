# Concurrency

### Primitives

- Atomics - 
- Mutex - 
- Condition variables - 

```rs
use std::sync::atomic;
```

Availability of Atomic types depends on hardware architecture and OS. Most OSes provide atomic types up to a pointer size (size depends on system architecture).

Modifications to Atomics are possible through a shared reference.

### Memory ordering

Two threads could potentially see operations on different variables in a different order.

For instance, one thread writes to one the another variable, a different thread might see this operation order in reverse.

`Relaxed` is the simplest variant of memory ordering with fewest guarantees.

### Thread-local storage (TLS)

A memory management method that uses static or global memory local to a thread. Every thread as its own local copy of the variable. Unlike global variables, TLS are only visible to the local thread.

https://learn.microsoft.com/en-us/cpp/c-language/thread-local-storage?view=msvc-170
