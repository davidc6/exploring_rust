
// lifetime is a kind of generic (a type that is specified at a later stage which eliminates repetition for each type)
// lifetimes ensure that references are valid as long as we need them to be
// every reference has a lifetime and this lifetime is the scope for which this reference is valid
// lifetimes mostly are implicit and inferred just like types
// we must annotate lifetimes of references could be related (just like with types if multiple types are possible)
// we are required to annotate types using generic lifetime parameters to ensure that references will be valid at runtime
// the main aim of lifetimes is to prevent dangling pointers

// static lifetime denotes that the reference can live for the entire duration of the program
// for instance string literals all have the 'static lifetime
// let some_word: &'static str = "hi"; -> the text of the sting is stored directly in the binary

// x: &'a str binds lifetime variable 'a to the lifetime of x; param x is a ref to an str with lifetime a
// y: &'b str binds lifetime variable 'b to the lifetime of y; param y is a ref to a str with lifetime b
fn longest_word<'a, 'b>(x: &'a str, y: &'b str) -> String {
    if x.len() > y.len() {
        let mut w = String::from(x);
        w.push_str(y); // y is equvalent to &*y
        w
    } else {
        let w = String::from(y);
        w
    }
}

fn main() {
    let x = String::from("hello");
    let result;

    {
        let y = String::from("hi");
        result = longest_word(x.as_str(), y.as_str()); 
    }

    println!("{}", result);
}
