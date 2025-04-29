# Algorithmic exercises

## General notes

Running exact tests:

```sh
$ cargo test -- --exact <full_name_of_test>
```

## Common algorithmic patterns

1. Sliding Window
2. Two pointer
3. Slow and fast pointers
4. Linked List Reversal
5. Binary Search
6. Top K Elements
7. Binary Tree Traversal
8. Graph and Matrices
9. Backtracking
10. Dynamic Programming
11. Bit Manipulation 
12. Overlapping intervals
13. Monotonic Stack
14. Prefix Sum

## Clock (resources)

- [Div Euclid & Rem Euclid](https://notes.statn.dev/languages/rust/div_rem/div_euclid_rem_euclid.html)
- [Understanding % and rem_euclid() in Rust](https://www.iainmaitland.com/remainder)

## General knowledge

Defer coercion - make smart pointer types behave as much like the underlying value as possible.

For example, Box<Sometype> is mostly like using Sometype.

A type is being coerced to behave like another.

```
pub trait Deref {
    // associated type, a placeholder for concrete type that must be implemented
    type Target;

    fn deref(&self) -> &Self::Target;
}

impl Deref for String {
    type Target = str;

    fn deref(&self) -> &str {
    }
}
```

DST - dynamically sized type, these include additional information (fat pointer) like length. let string = String::with_capacity(5); let s = &string[1..];
A type is Sized if the size is known at compile time (not a DST). Examples are u32, String, bool. str is not Sized. It is also a marker trait (no methods to implement). It is also an auto trait, it is automatically implemented by the compiler. To opt out of Sized, we need to put ? in front of it i.e. ?Sized. 

From and Into are dual traits Into <> From

pub struct WrappingU32 {
    value: u32
}

impl From<u32> for WrappingU32 {
    fn from(val: u32) -> Self {
        WrappingU32 {
            value: val
        }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}

There can only be one target type for a given type (i.e. String can only deref to str). An associated type is uniquely determined by the trait implementation. Deref cannot be implemented more than once you can only specify one Target for a given type. 

From<T> can be implemented multiple times for a type, as long as the input type T is different. This works since these are considered differnet traits. 

```
// RHS is a generic parameter, defaults to Self (whatever type this is being implemented for)
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add<u32> for u32 {
    type Output = u32;

    fn add(self, rhs: u32) -> Self::Output {
        self.0 + rhs
    }
}
```

Ownership - each value has an owner
Borrowing - 
Lifetimes - 

## Todos

- [] LRUCache to use Arc<Mutex<_>> or Arc<RwLock<_>> instead

## General resources

- [Rust debugger setup](https://gist.github.com/xanathar/c7c83e6d53b72dd4464f695607012629)
