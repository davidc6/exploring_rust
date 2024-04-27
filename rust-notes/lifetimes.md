# Lifetimes

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

