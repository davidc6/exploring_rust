# Error Handling

- Necessity to know about the error details in order to interact with it
- Enumerate errors in order to enable the user to recover by differentiating errors
- Prefer to implement Error trait for custom error types

### 

### Custom errors / enumeration

Provide error variants. Each variant includes error cause.

```rs
pub enum ConnectionError {
  Server(std::io::Error),
  Client(std::io::Error)
}
```

Each custom error should implement the Error trait where it gets all the
common error methods. Additionally each type should implement Debug and Display traits
which enables callers to print errors. Display is usually for one liner errors and
Debug for more descriptive ones. Errors should also implement Send and Sync traits to
enable to use errors between threads and should be static where possible in
order to easily propagate errors up he call stack without running into lifetime
issues. 

- Move on
- Single opaque error
- Severely type erased error - 

### Trait objects

```rs
Box<dyn Error + Send + Sync + 'static>
``` 

which does not reveal anything about the error apart from the fact that it is an error.
Whether to make the error opaque or not is a matter of whether there is any
useful error information.

### Crates

- [thiserror](https://docs.rs/thiserror/latest/thiserror/)

Used in libraries to create custom error types that implement `From<T>`

- [anyhow](https://docs.rs/thiserror/latest/thiserror/)

Used in applications to help with error handling in functions.

### Additional context

`?` - question mark operator is used to reduce boilerplate. This operator handles Error (Err) side of things and returns `return Err(...)` expression. 

### Further remarks

- Errors can be encoded as a trait object and this avoids the need for an enum variant
- This erases the detail of the specific error types
- The receiver has access to the Error trait and the trait bounds
- The receiver would not know the original static type of the error
- Rust has coherence rules
  - For any given type and method there is only one correct choice for which implementation of that method to use for that type