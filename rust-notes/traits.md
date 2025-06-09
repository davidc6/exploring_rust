# Traits

## Polymorphic code

```rs
// Trait objects: reference to any value that implements a given set of methods
&dyn Any, &mut dyn Read
```

Trait object - is a fat pointer, a reference to a value that implements a certain trait. It carries a value's address and a pointer to the trait's implementation appropriate for that value.

```rs
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;

trait Log {
    fn send(&mut self, timestamp: u64) -> Result<(), Box<dyn Error>>;
}

struct Logger {}

impl Log for Logger {
    fn send(&mut self, timestamp: u64) -> Result<(), Box<dyn Error>> {
        println!("Logging {timestamp}");
        Ok(())
    }
}

// We use a logger here without caring about the type of logger.
// The argument that this function takes is a mutable reference
// to a any value that implements the Log trait
fn log_timestamp(log: &mut dyn Log) -> Result<(), Box<dyn Error>> {
    let system_time = SystemTime::now();
    let duration = system_time.duration_since(UNIX_EPOCH)?;
    log.send(duration.as_secs())?;
    Ok(())
}

fn main() {
    let mut logger = Logger {};
    log_timestamp(&mut logger);
}

trait Log {
    fn send(&mut self, timestamp: u64) -> Result<(), Box<dyn Error>>;
}
```

Calls through `&mut dyn Log` incur the overhead of a dynamic dispatch (aha virtual method call). `dyn Log` is a trait object in the example above. However, we cannot just write `dyn Log` since the compiler needs to know the size at compile time and types that implement `Log` can be of any size. References in Rust are explicit ( `&mut dyn Log`). A reference to a trait type is called *trait object*. A trait object points to some value, has a lifetime and can be either shared or mut. 

A trait object includes extra information (metadata) about the referent type. This is used by Rust behind the scenes to dynamically call the right method depending on the type. Rust does not support downcasting from &mut dyn to the actual type.

## Marker traits

Used to bound generic type variables which can express constraints that can't be captured otherwise. Example of these traits: Sized and Copy. The compiler uses these to mark certain types as "of interest". 

T: ?Sized - type bound on a type variable

## Borrow and BorrowMut

By implementing the `Borrow<T>` trait, the borrow method efficiently borrows `&T`. There are a number of restrictions that `Borrow<T>` imposes. `&T` should hash and compare the same way as the value it has borrowed from. This is very useful when dealing with hash tables and trees when hashing or comparing.

`String` type implements `AsRef<str>`, `AsRef<[u8]>` and `AsRef<Path>`. These will have different hash values. 

```rs
trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}
```

