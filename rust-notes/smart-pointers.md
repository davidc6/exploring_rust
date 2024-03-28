# Smart pointers

## Pointers and references

When a new variable binding is created, a name is given to a particular value that is stored at a particular location on the stack. 

```rs
// 3 is stored at some (made-up here) location 0xd3h045 in memory
// some_number corresponds to some memory location which holds the value 3, and when we refer to some_number we get 3
let some_number = 3; 
```

Pointers are essentially variables that contain an address in memory. The address itself points at some other data.

In Rust most simplest and common pointer is a reference (`&`). References essentially "borrow" the value they point to i.e. they refer to the data. References are non-owning pointers, meaning that they do not own the value. The original owner remains the sole owner of the original value. 

```rs
let some_number = 3;                    // location 0xd3h045, value 3
let some_number_2 = 4;                  // location 0xd3h047, value 4
let some_number_2_ref = &some_number_2; // location 0xd3h040, value 0xd3h047 <----- value is the location in memory

println!("{:p}", some_number_2_ref);

// pointers can be dereferenced i.e. accessing value at that location in memory
println!("{}", *some_number_2_ref);
```

If memory on the heap is allocated, we need a pointer to the allocated memory (that's how `malloc` works in C). A pointer is required if there's a structure that can change in size. 

We follow the reference to an address in memory to access the data stored at that address. The data is owned by a variable. 

```rs
fn main() {
    let name = String::from("Mary");

    // Here the name variable (ref name) is passed as a reference to to_bytes function
    // At run time a reference to &name is a single machine word
    // which holds the address of the "name" (could be on the stack or heap).
    // In Rust this is called to "borrow a reference to name".
    // 
    // Given this reference name (&name), *name refers to the value that name points to.
    // This is very similar to C / C++ operators - & and *.
    //
    // Same as in C, these references do not auto-free any resources once out of scope.
    // In Rust however, each value ownership and lifetime is tracked so there's no way to 
    // produce a null pointer (at least in safe Rust).
    //
    // 
    let string_as_bytes = to_bytes(&name);

    println!("Name {:?} as bytes: {:?}", name, string_as_bytes);
}

fn to_bytes(name: &String) -> &[u8] {
    name.as_bytes()
}
```

When an owning pointer is dropped, it takes referent with it. For example, when String type is dropped, all references are dropped too. 

```rs
let mut numbers = vec![3, 1, 2, 5];

// sort() requires in-place mutation of numbers
// therefore it needs a mutable access to the values
// (&mut number) is the same as the below
numbers.sort();

// Same as above but more verbose
(&mut numbers).sort();
```

## Creating a smart pointer

```rs
use std::ops::Deref;

// Define a tuple struct Container that is generic over T
// The struct have a generic type parameter T since it will hold value of any type
struct Container<T>(T);

// Container has one function (new(value: T)) which takes one parameter of type T
// and returns an instance of Container which holds the value.
impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container(value)
    }
}

// In order to dereference Container, Deref needs to be implemented on Container.
impl<T> Deref for Container<T> {
    // Associated type needs to be defined to use Deref trait.
    type Target = T;

    // deref method is filled with &self.0 which returns a reference to the value
    // that will be accessed using * (deref operator)
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let number = Container::new(5);
    // same as
    // *number is the same as *(number.deref()) <--- this is due to ownership system, since ownership of T should not be taken
    assert!(5 == *number);
}
```

### Deref coercion

This applies to types that implement the Deref built-in trait. 

Auto conversions &String to &str, &Vec<u32> to &[u32], &Box<SomeType> to &SomeType. The idea behind Deref trait it to make smart pointer types behave the same as underlying structure.

```rs
fn main() {
    let greeting = Container::new(String::from("Hello"));
    // Here deref coercion enables Container to behave like the underlying type
    // 
    // Without deref coercion, it would be something like:
    // greet(&(*greeting)[..])
    // 1. Deref (*greeting) to into a String
    // 2. Take a string slice of the String (whole string)
    greet(&greeting);
}

fn greet(greeting: &str) {
    println!("{}, stranger!");
}
```




