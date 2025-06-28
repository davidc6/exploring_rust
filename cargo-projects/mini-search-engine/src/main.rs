use std::fs::read_to_string;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

/// Attempts to read all the files in a given directory and
/// send the content of each file on the channel.
///
/// Exits if a file cannot be read. If all operations are successful,
/// returns a tuple - a Receiver and JoinHandle.
fn file_reader(files: Vec<String>) -> (Receiver<String>, thread::JoinHandle<std::io::Result<()>>) {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        for file in files {
            let content = read_to_string(file)?;

            let Ok(()) = tx.send(content) else {
                break;
            };
        }

        Ok(())
    });

    (rx, handle)
}

fn main() {}
