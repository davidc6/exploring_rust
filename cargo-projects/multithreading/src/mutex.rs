use std::{sync::Mutex, thread};

pub fn simple_mutex() {
    let some_data = Mutex::new(String::from("Hello"));

    thread::scope(|s| {
        s.spawn(|| {
            let mut data_guard = some_data.lock().unwrap();
            *data_guard += ", World!";
        });
    });

    // Remove Mutex protection around the String by using into_inner()
    println!("{:?}", some_data.into_inner().unwrap());
}
