fn main() {
    // immutable borrow - uncomment to test
    // let letters = vec!["a", "b", "c"];
    // println!("Before: {:?}", letters);

    // let borrow = || println!("Closure: {:?}", letters);

    // println!("Before: {:?}", letters);
    // borrow();
    // println!("After: {:?}", letters);

    // mutable
    let mut letters = vec!["a", "b", "c"];
    println!("Before: {:?}", letters);

    // move keyword can be used to force the closure to
    // take ownership of the values it uses. This is useful when passing
    // a closure to a new thread to move the data so it's owned by a new thread
    let mut borrow = || letters.push("d");

    // println is not possible because no other mutable borrows are allowed,
    // before the mutable borrow ends

    borrow();
    println!("After: {:?}", letters);
}