# Notes

## Tooling

- `cargo` - compilation manager
  - `cargo new hello-world` - creates new app / set up a new app
- `rustup` - toolchain manager
- `rustc` - Rust compiler

## Syntax / library

- `!` - macro invocation e.g. `println!()`
- `panic` - abrupt termination; allows the program to terminate immediately; should be used when a program reaches unrecoverable state
- `env.args()` - an iterator, produces each value on demand
- `traits` - a collection of methods (behaviour) that a type implements/can implement
  - when importing a trait must be in scope to use its methods
- `b"world"` - byte literal is used to indicate that this is a byte string (sequence of bytes)

### Types

- `u8 (0 - 255)`, `u16 (65535)` etc - basic integer types
  - `u8` - 1 byte / 8 bits
  - `u16` - 2 bytes / 16 bits
- `unit type` / `()` - formally a zero-length tuple, this empty value is used when there is no other significant value that can be returned. Fns that appear to not return any value and expressions that are terminated with a semicolon return `()`.

### Strings

- If dealing with unicode test - `String` and `&str`
- For filenames - `use::path::PathBuf` & `&Path`
- Non UTF-8 binary data - `Vec<u8>` and `&[u8]`
- Env var names and command-line args (presented by OS) - `OsString` and `&OsStr`
- C interop with null terminated strings - `std::ffi::CString` and `&CStr`
- Other String-Like Types (refer to Programming Rust 2nd ed)

### Array, Vector, Slice

- Array (contiguos block of memory, fixed size)
```rs
let ar: [i32, 4] = [1, 8, 9, 12]
```
- Vector (dynamic allocation)
```rs
let ve: Vec<i32> = vec![1, 8, 9, 12]
```
- Slice (temporary view into an array or vector)
```rs
let ar: [i32, 4] = [1, 8, 9, 12]
let sl = &ar[1..3]
```

### Array

`let a = [1,2,3]` - stack allocated stack

`let a: &[u32] = &[1, 2, 3]` - slice, which also store references and length of 
things they point to

`let a = vec![1, 2, 3]` - contiguous growable array, allocated on the heap, reference 
to it is stored in the stack 

### Ownership

- Stack - static memory allocation (we can't modify context of this area; fixed size; high memory addresses; allocation is quicker)
- Heap - dynamic memory allocation (not fixed and can vary at runtime; size is dependent on the system; lower memory addresses; allocation is slower as have to find first)

- Ownership:
  - Keeps track of what parts of code are using what data on the heap
  - Minimizes duplicate data on the heap
  - Cleans up used data on the heap (to keep clean up unused space)
  - Each value has a variable called owner (only one at a time)
  - When owner is out of scope, value is dropped
  
- Borrowing - when a value has an owner and another function borrows the value from the owner
- Referencing - when original value is references somewhere else in the code (even when that original variable is out of scope). References are immutable by default (but can be made mutable)
  - One can only have either one mutable reference or any number of immutable references
  - References must be valid

- Immutable by default can prevent data race at compile time
- A data race is like race condition can occur when:
  - Two or more pointers access the same data at the same time
  - At least one of the pointers is being used to write to the data
  - There's no mechanism being used to sync access to data

- Slice enables us to reference a contiguous seq of elements rather than the whole collection
  - It is a sort of a reference and does not have ownership 
  
- Rust breaks the deadlock between safe (garbage collectoin - Haskell, JavaScript, Ruby etc) and performance (manual memory management - C, C++) and restricts how a program uses pointers. Rust's compiler checks for safety errors such as dangling pointers, double frees, using and utilizing memory and so on. Pointers are addresses in memory at compile time but Rust proofs that your code is safe.

- In C / C++ an instance of some class owns some other object that it points to. Owning object gets to decide when to free the owned object, once the owner is destroyed it destroys its possessions. It is therefore up to you to make sure that you don't point to that object anymore.

- In Rust when the owner gets dropped the owned value is dropped too.

- Variables own their values, structs own their fields, tuples, arrays, vectors own their elements

- Values can be moved from one owner to another (allows to build, rearrange and tear down tree - owners and their owned values)
- Simple types like ints, floats, chars are excluded from the ownership rules as they are `Copy` types
- `Rc` and `Arc` are types provided by the standard library that allows values to have multiple owners under certain conditions
- One can borrow a reference to a value and references are non-owning pointers with limited lifetimes

#### Moves

- Assigning value to a var, passing it to fn / returning it from fn does not copy a value it moves it
- Source passes ownership of a value to destination and destination controls the value's lifetime

#### Tips

- Use references where full ownership is not required.
- Duplicate the value
- Reduce the number of long-lived objects
- Data should be wrapped in a type that is designed to assist with movement issues 
  
#### Lifetimes

- Every reference has a lifetime (the scope for which the reference is valid)
- Same as types, lifetimes are inferred most of the times
- Lifetimes allow us to prevent dangling references

### Error handling

`Box<dyn Err>` - dynamic error, allows us to handle error of various types; dyn highlights the fact that calls on the associated Trait are dynamically dispatched. This relies on the fact that all errors implement Error trait (which is not always the case). "Boxing" an error means that we can store it 
somewhere (heap) and hold a pointer to that location.

### Attributes

`#[derive(Debug)]` - this is some metadata that is applied to some module, crate or item. For instance,
`[#cfg(test)]` allows to run tests only when we run `cargo test` command.

### Pointer types

- references - 
  - &String - reference to a string value
  - &i32 - reference to a 32 bit integer
  - at run time a ref to i32 is a machine word holding the address to i32
  - &x - "borrows a reference to x"
  - expression *r - refers to value r points to
  - a reference does not automatically free any resources when it goes out of scope
  - no null references in Rust
  - no dangling pointers, double frees and pointer invalidation as Rust tracks ownership and lifetimes of values
  - &T is an immuttable shared reference which one can have many of
  - &mut T is a mutable exclusive reference; can read and write value it points to; as long as this this reference lives you cannot have any other references to it; single writer or multiple readers
- boxes - 
  - allocate value on the heap
    ```rs
        let a = ("Tom", 34);
        // b is Bob<(&str, i32)>
        let b = Box::new(a);
    ```
  - allocated enough memory to contain the tuple on the heap
  - when `b` goes out of scope memory get freed unless moved
  - moves are essential to thwe way Rust handles heap-allocated values
- unsafe (raw) pointers - 
  - raw pointer types are just like pointers in C/C++
  - `*mut T`, `*const T` - are unsafe
  - What makes them raw is that Rust makes no effort to track what they point to
  - They can be null or point to memory that has been freed / contains value of a diff type
  - raw pointers can only be dereferenced in `unsafe` block
  - `unsafe` block is Rust's way to allow access to advanced features where safety is up to you
  - 

### Functional programming (features)

- **Closures** - functional like construct that you can store in a variable
- **Iterators** - a style of processing series of elements

### Misc

`impl` - implementation block, allows to define methods for a type
  - `&self` - within `impl` &self is alias for the type that impl block is for
  
`::` - to access module path
`?` - allows for concise error handling, unpacks Result type if result is OK, error if Error
`expressions vs statements` - https://nickymeuleman.netlify.app/garden/rust-expression-statement
`usize` - how many bytes it takes to reference a location in memory. On 32 bit architecture it is 4 bytes on 64 bit is 8 bytes. It is pointer-sized. If 

machine word - address size value on the machine (the code runs on)
something is 2 words long - each word is either 8 / 16/ 32 / 64 bits
word length - data bus width, how much data can be transfered at each time

### Commands

`$ rustc --crate-name sieve --crate-type lib main.rs` - emit create that is a library
`$ rustc --crate-name sieve main.rs --test` - emits tests harness

## Examples

- [Greatest common divisor](./greatest-common-divisor/main.rs)
- [Structs, implement and traits](./struct-impl-trait/main.rs)
- [Sieve of Erathosthenes](./sieve-of-erathosthenes/main.rs)
- [CubeSat](./cube-sats/main.rs)

## Reference material

- Convert string to int -  https://programming-idioms.org/idiom/22/convert-string-to-integer/1163/rust
- What are the differences between [], &[], and vec![]? - https://stackoverflow.com/questions/57848114/what-are-the-differences-between-and-vec
- Clippy - https://github.com/rust-lang/rust-clippy
- Impls & Traits - https://learning-rust.github.io/docs/b5.impls_and_traits.html
