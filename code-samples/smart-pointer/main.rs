use std::ops::Deref;

fn main() {
    struct MyPointer<T>(T);

    impl<T> MyPointer<T> {
        fn new(val: T) -> MyPointer<T> {
            MyPointer(val)
        }
    }

    impl<T> Deref for MyPointer<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyPointer::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
