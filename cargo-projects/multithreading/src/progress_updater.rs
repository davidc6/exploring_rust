use rand::{thread_rng, Rng};
use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn process_item(item: usize) {
    println!("Working on {item}");

    let mut rng = thread_rng();
    let random_sleep_time = rng.gen_range(0..=3);

    thread::sleep(Duration::from_secs(random_sleep_time));
}

pub fn progress_updater() {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    let th = thread::spawn(|| {
        for count in 0..100 {
            process_item(count);
            COUNT.store(count + 1, std::sync::atomic::Ordering::Relaxed);
        }
    });

    loop {
        let current_item = COUNT.load(std::sync::atomic::Ordering::Relaxed);
        if current_item == 100 {
            break;
        }
        println!("Processing, {current_item} out of 100 done");
        thread::sleep(Duration::from_secs(1));
    }

    th.join().unwrap();
}
