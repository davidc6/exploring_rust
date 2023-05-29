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
impl Process for String {
    fn replace(&self) {
        println!("Replacing something in String");
    }
}

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

```rs
fn call_replace(data: impl Process) {
    data.replace();
}
```

This is just syntactic sugar and is the same as

```rs
// read as: call_replace function is generic over T which is bound by Process trait
fn call_replace<T: Process>(data: T) {
    data.process();
}
```

The compiler will then generate code for each type that is called in the code. Let's take `str` as an example.

```rs
// this code will call line 22 in this code example
fn call_replace_str(data: &str) {
    data.replace();
}


How about calling the same method a each element on a collection?

```rs
