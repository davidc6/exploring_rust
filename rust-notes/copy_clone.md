# Copy & Clone

# Copy

- Stack-allocated can be easily copyable
- Not moved but individually copied
- All types that do not require allocations implement (or can implement) `Copy`
- `Copy` allows to duplicate a value by copying only the parts that are stored on the stack
- There's no custom logic involved in the copy process (the process does not care of heap allocation and ownership etc)
- A type that implements `Copy` must also implement `Clone`, since a `Copy` type also has a trivial `clone()` implementation
- Allows for values to be copied implicitly when passing to 
- `Copy` trait is a subtrait of `Clone`
- Shallow, bitwise copy of the value (memory representing the value is copied as-is, byte for byte)

```rs
let a = 1;
// a is not moved into incr_by_one()
let b = incr_by_one(a);

// a is still in scope as it was copied
```

- Types that implement this trait make a bitwise (bit by bit) copy of the value
- For example, scalar types such as integers are copied

```rs
let int_one = 1;
// same as let int_two = int_one.clone();
let int_two = int_one; // copy of the value is made here
// int_one is still in scope

// i is a copy here
fn int_here(i: u8) -> u8 {
    i + 1
}

int_here(int_one);
```

- Non-scala types (i.e. String) have to be cloned

```rs
let s_one = String::from("one");
let s_two = s_one.clone();
```

# Clone

- Types that implements this trait allow you to make a deep copy of a value
- Used to deep clone value allocated on the heap and not just the stack
- Code to duplicate the value that implements clone frequently involves running custom logic and copying heap data
- Deriving `Clone` implements `clone()` method. All fields and values should implement `Clone` too
