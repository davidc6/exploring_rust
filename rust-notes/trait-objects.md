# Trait objects

Let's define a trait.

```rs
// trait definition
trait Process {
    fn replace(&self);
}
```

Now let's implement it for String and str types.

```rs
// #1
impl Process for String {
    fn replace(&self) {
        println!("Replacing something in String");
    }
}

// #2
impl Process for str {
    fn replace(&self) {
        println!("Replacing something in str");
    }
}
```

We can now call the new method on these two types.

```rs
fn main() {
    "test String".to_string().replace();
    "test str".replace();
}
```

## Static dispatch

Let's define a generic function

```rs
fn call_replace(data: impl Process) {
    data.replace();
}
```

This is just syntactic sugar and is the same as

```rs
// read as: call_replace function is generic over T which is bound by Process trait
// or <T: Process> - for any type that implements Process trait
fn call_replace<T: Process>(data: T) {
    data.process();
}
```

Somehow the compiler needs to generate code and call `.process()`. It does not know the type of T.
This is known as monomorphisation. The compiler will then generate code for each type that is called in the code 
since it knows what type actually calls it.

Let's take `str` as an example.

```rs
// concrete implementation
call_replace(&"test");

// the compiler will generate similar code which will call #2 code example
fn call_replace_str(data: &str) {
    data.replace();
}
```

The same will be applied to String or any type that get called in the code. This is known as static dispatch. The code gets generate by the compiler statically at compile time.



How about calling the same method a each element on a collection?

```rs
// this function takes in a single iterator that has items of one type T
// call_replace_iter is generic only one type T which should implement Process trait
// in this example it is a slice
fn call_replace_iter<T: Process>(collection: &[T]) {
    for item in collection {
        item.replace();
    }
}

// we cannot really have this
call_replace_iter(&["str", String::from("String")]),
```

We cannot create a vector/array/slice of heterogeneous (mixed/diverse) type.

Let's 

```rs
let collection: Vec<Box<dyn Process>>;
```

```rs
struct Payment {
    fn execute(&self);
}

struct Payout<T: Payment>{
    payments: Vec<T>
}

impl<T> Payout<T>
where
    T: Payment,
{
    pub fn process(&self) {
        for payment in self.payments.iter() {
            payment.execute();
        }
    }
}

struct PayPal {
    pub email: String
}

impl Payment for PayPal {
    fn execute(&self) {
        // logic to execute this payment
        println!("Executing payment via PayPal")
    }
}
```