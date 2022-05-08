# Notes

## Tooling

`cargo` - compilation manager
  - `cargo new hello-world` - creates new app / set up a new app
`rustup` - toolchain manager
  
`rustc` - Rust compiler

## Syntax / library

`!` - macro invocation e.g. `println!()`

`panic` - abrupt termination

`env.args()` - an iterator, produces each value on demand

`traits` - a collection of methods that types implement
  - when importing a trait must be in scope to use its methods

`b"world"` - byte literal is used to indicate that this is a byte string (sequence of bytes)

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

### Misc

`::` - 

`impl` - implementation block, allows to define methods for a type
  - `&self` - within `impl` &self is alias for the type that impl block is for
  
`::` - to access module path

### Commands

`$ rustc --crate-name sieve --crate-type lib main.rs` - emit create that is a library
`$ rustc --crate-name sieve main.rs --test` - emits tests harness

## Examples

- [Greatest common divisor](./greatest-common-divisor/main.rs)
- [Structs, implement and traits](./struct-impl-trait/main.rs)
- [Sieve of Erathosthenes](./sieve-of-erathosthenes/main.rs)

## Reference material

- Convert string to int -  https://programming-idioms.org/idiom/22/convert-string-to-integer/1163/rust
- What are the differences between [], &[], and vec![]? - https://stackoverflow.com/questions/57848114/what-are-the-differences-between-and-vec
- Clippy - https://github.com/rust-lang/rust-clippy
- Impls & Traits - https://learning-rust.github.io/docs/b5.impls_and_traits.html
