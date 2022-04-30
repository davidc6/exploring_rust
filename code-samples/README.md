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

`::` - 

`impl` - implementation block, allows to define methods for a type
  - `&self` - within `impl` &self is alias for the type that impl block is for

## Examples

- [Greatest common divisor](./greatest-common-divisor/main.rs)
- [Greatest common divisor](./struct-impl-trait/main.rs)

## Reference material

- Convert string to int -  https://programming-idioms.org/idiom/22/convert-string-to-integer/1163/rust
- What are the differences between [], &[], and vec![]? - https://stackoverflow.com/questions/57848114/what-are-the-differences-between-and-vec
- Clippy - https://github.com/rust-lang/rust-clippy
