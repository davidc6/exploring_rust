# Lifetimes

- A lifetime is a form of generic (generic parameter)
- A lifetime tracks *time* and not *type*
- A lifetime generic is not the same as type generic
- Represents a scope of validity for a reference
- You can think of them as "regions of memory during which references are valid"
- Used by the compiler to verify borrowing rules
- Used by the compiler to ensure that references do not outlive the data they point to
- The main idea is to prevent dangling references
- Lifetime annotation does not change how long the reference lives, it just describes the relationship of the lifetimes to each other
- Ensure that references are valid as long as they are needed to be
- Every reference (&) in Rust has a lifetime
- Generally lifetimes are implicit (just like types)
- Lifetimes should be annotated when the lifetimes of references could be related in a number of different ways
- Generic lifetime parameters should be used to ensure actual references are valid at runtime

- Assigning value to a variable, passing to a function or returning from a function "moves" value

```rs
let v = vec!["hello", "world"];
let v1 = v;
let v2 = v;
```

This example will error since v is moved once and cannot be moved again.

There are reserved lifetime names such as `'static`. This lifetime means that data pointed to my the reference lives for the lifetime of the program.

## Example 1

```rs
// This function takes in a string slice that is valid for some lifetime 'a as well as start and end indices of type usize.
// The function then returns another string slice that is guaranteed to be valid fot the same lifetime 'a.
fn str_slice_slicer<'a>(value: &'a str, start: usize, end: usize) -> &'a str {
    &value[start..end]
}
```

## Example 2

```rs
// TODO
```
