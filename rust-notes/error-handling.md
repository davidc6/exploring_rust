# Error Handling

- Necessity to know about the error details in order to interact with it
- Enumerate errors in order to enable the user to recover by differentiating errors

## Custom errors

Provide error variants. Each variant includes error cause.

```rs
pub enum ConnectionError {
  Server(std::io::Error),
  Client(std::io::Error)
}
```

Each custom error should implement Error trait where it gets all the
common error methods. Additionally each type should implement Debug and Display traits
enabling callers to print errors. Display is usually for one liner errors and
Debug for more descriptive ones. Errors should also implement Send and Sync traits to
enable to use errors between threads and should be static where possible in
order to easily propagate errors up he call stack without running into lifetime
issues. 

- Move on
- Single opaque error
- Severely type erased error - 

```rs
Box<dyn Error + Send + Sync + 'static>
``` 

which does not reveal anything about the error apart from the fact that it is an error.
Whether to make the error opaque or not is a matter of whether there is any
useful error information.
