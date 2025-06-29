use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::mpsc::{channel, Receiver};
use std::thread::{self, JoinHandle};

#[derive(Default)]
struct InvertedIndex {
    index: HashMap<String, HashMap<String, usize>>,
}

impl InvertedIndex {
    fn from_value(id: usize, value: String) -> Self {
        InvertedIndex::default()
    }
}

/// Attempts to read all the files in a given directory and
/// send the content of each file on the channel.
///
/// Exits if a file cannot be read. If all operations are successful,
/// returns a tuple - a Receiver and JoinHandle.
fn file_reader(files: Vec<String>) -> (Receiver<String>, JoinHandle<std::io::Result<()>>) {
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

fn file_indexing(content: Receiver<String>) -> (Receiver<InvertedIndex>, JoinHandle<()>) {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        for (id, value) in content.into_iter().enumerate() {
            let index = InvertedIndex::from_value(id, value);

            let Ok(_) = tx.send(index) else {
                break;
            };
        }
    });

    (rx, handle)
}

fn main() {}
