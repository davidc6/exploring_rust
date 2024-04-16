use std::{sync::atomic::AtomicBool, thread, time::Duration};

fn do_work() {
    println!("Working");
    // Put the current thread to sleep for 5 seconds
    thread::sleep(Duration::from_secs(5));
}

pub fn stop_flag() {
    // "static" variable that lasts for the entirety of the program.
    static STOP: AtomicBool = AtomicBool::new(false);

    // Spawn a background thread that does some work by calling do_work function
    // while STOP variable is not set to "false".
    let thread = thread::spawn(|| {
        // Load value from the atomic boolean and pass in Relaxed memory ordering.
        while !STOP.load(std::sync::atomic::Ordering::Relaxed) {
            do_work();
        }
    });

    // Here, we listen for user input on the main thread.
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "h" => println!("Possible commands: h, s"),
            "s" => break,
            cmd => println!("This command is not known {cmd:?}"),
        }
    }

    // Once there is an input from standard input,
    // store new value into the atomic boolean.
    STOP.store(true, std::sync::atomic::Ordering::Relaxed);

    // Wait for the background thread to finish.
    // In this example, main thread waits for the background thread to be finished.
    thread.join().unwrap();
}
