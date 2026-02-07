use std::io::Result;
use std::sync::{atomic::AtomicBool, Arc};
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;
use std::time::Duration;
use inotify::{EventMask, Inotify, WatchMask};

fn main() -> Result<()> {
    let is_running = Arc::new(AtomicBool::new(true));
    let path_to_watch = "/var/log";
    let mut inotify = Inotify::init().expect("Failed to initialise inotify");

    inotify
        .watches()
        .add(
            path_to_watch,
            WatchMask::CREATE | WatchMask::MODIFY | WatchMask::DELETE
        )
        .expect("Failed to add watch");

    println!("Watching on {path_to_watch} started");

    let mut buf = [0u8; 4096];

    while is_running.load(Relaxed) {
        // We need at least one event here hence blocking.
        let events = match inotify.read_events_blocking(&mut buf) {
            Ok(e) => e,
            Err(e) => {
                eprint!("Error {:?}", e);

                // Sleep for a very short period of time to avoid rapid error loop.
                sleep(Duration::from_millis(50));
                continue;
            }
        };

        println!("Hello");

        for event in events {
            let event_name = event.name.and_then(|n| n.to_str()).unwrap_or("<UNKNOWN>");

            if event.mask.contains(EventMask::CREATE) {
                println!("CREATE: {}/{}", path_to_watch, event_name);
            }
            if event.mask.contains(EventMask::DELETE) {
                println!("DELETE: {}/{}", path_to_watch, event_name);
            }
            if event.mask.contains(EventMask::MODIFY) {
                println!("MODIFY: {}/{}", path_to_watch, event_name);
            }
        }
    }

    Ok(())
}
