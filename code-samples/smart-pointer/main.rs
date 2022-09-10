use std::ops::Deref;

fn main() {
    // define struct and declare a generic parameter T
    // to hold value of any type
    struct MyPointer<T>(T);

    impl<T> MyPointer<T> {
        fn new(val: T) -> MyPointer<T> {
            MyPointer(val)
        }
    }

    // T is a generic type parameter
    impl<T> Deref for MyPointer<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            // the value we want to access using the deref operator - *
            &self.0
        }
    }

    let x = 10;
    let y = MyPointer::new(10);

    assert_eq!(10, x);
    assert_eq!(10, *y);
}
