# Traits

A trait is some kind of a feature that any given type might (or might not support). Traits are similar to interfaces in the other languages. Trait methods are similar to virtual methods in languages like C++ or C#. 

## Polymorphic code

```rs
// Trait objects: reference to any value that implements a given set of methods
&dyn Any, &mut dyn Read
```

Trait object - is a fat pointer, a reference to a value that implements a certain trait. It carries a value's address and a pointer to the trait's implementation appropriate for that value. Each trait object takes up two machine words (pointer to the value and implementation).

### v-table

Vtable or virtual table is generated at compile time and shared by all objects of the same type. 



```rs
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;

trait Log {
    fn send(&mut self, timestamp: u64) -> Result<(), Box<dyn Error>>;
}

struct S3Logger {
    name: String
}

impl Log for S3Logger {
    fn send(&mut self, timestamp: u64) -> Result<(), Box<dyn Error>> {
        println!("Logging to {}: {timestamp}", self.name);
        Ok(())
    }
}

struct FileLogger {
    name: String,
    location: String
}

impl FileLogger {
    fn new(location: &str) -> Self {
        FileLogger {
            name: "file".to_string(),
            location: location.to_string()
        }
    }
}

impl Log for FileLogger {
    fn send(&mut self, timestamp: u64) -> Result<(), Box<dyn Error>> {
        println!("Logging to {}: {timestamp}", self.name);
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
    let mut logger = S3Logger {
        name: "S3".to_string()
    };
    let _ = log_timestamp(&mut logger);

    let mut file_logger = FileLogger::new("/file/lives/here");
    let _ = log_timestamp(&mut file_logger);
}
```

Calls through `&mut dyn Log` incur the overhead of a dynamic dispatch (aha virtual method call). `dyn Log` is a trait object in the example above. However, we cannot just write `dyn Log` since the compiler needs to know the size at compile time and types that implement `Log` can be of any size. References in Rust are explicit ( `&mut dyn Log`). A reference to a trait type is called *trait object*. A trait object points to some value, has a lifetime and can be either shared or mut. 

A trait object includes extra information (metadata) about the referent type. This is used by Rust behind the scenes to dynamically call the right method depending on the type. Rust does not support downcasting from `&mut dyn` to the actual type.

## V-table (virtual table)

## Static vs dynamic dispatch

Most types in Rust implement `Sized` therefore they have a size that is know at compile-time. Trait objects and slices do not implement it. `dyn Log` or `[u8]` do not have a well-defined size. These are dynamically sized types (DSTs) as their size only gets know at runtime. The compiler should know the size of the type in order to know much space to allocate to it.

To solve this problem for the unsized types is to put them behind a fat (or wide) pointer which is `Sized`. It is like a normal pointer with the extra information. For a trait object the extra information is the pointer to the value itself and another to the v-table where the the trait method is implemented.

Static dispatch is when the address that the CPU needs to jump to is known at compile time. The address that we are dispatching to is known statically. For each type, a copy of implementation is created (the process is called monomorphization). Dynamic dispatch is when code can call a trait method without knowing what the type is. 

```rs
enum FileSystemType {
    Fat,
    Ntfs,
    Ext4,
    Hfs
}

struct FileSystem {
    fs_type: FileSystemType
}

impl FileSystem {
    // This is the same as
    // the caller must give should give two pieces of info here,
    // the address of Support and the address of is_fs_supported() method.
    pub fn check_support(&self, os: &dyn Support) -> bool {
        os.is_fs_supported(self.fs_type)
    }
}
```

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

