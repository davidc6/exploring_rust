use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::io;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::thread::{self, JoinHandle};

/// ID: { word_1: 1, word_2: 4, word_3: 2 }
///
/// [word_1]: { id_1: [1, 2], id_2: [4. 5] }
/// [word_2]: { id_1: [2, 3, 7], id_3: [6, 9] }
///
/// 1: { a: 1, b: 2, c: 4 }
/// 2: { c: 1, b: 3, f: 1 }
///
/// merged
///
/// An id is a unique identifier for each document in the corpus
#[derive(Default, Debug)]
struct InvertedIndex {
    index: HashMap<String, HashMap<Filename, Vec<usize>>>,
}

impl InvertedIndex {
    fn from_value(id: Filename, value: Content) -> Self {
        let mut index = InvertedIndex::default();

        let lines = value.0.lines();
        let mut count = 0;

        for line in lines {
            let split = line.split(' ');

            for word in split {
                count += 1;
                index
                    .index
                    .entry(word.to_owned())
                    .and_modify(|v| v.get_mut(&id).unwrap().push(count))
                    .or_insert_with(|| {
                        let mut hm = HashMap::new();
                        hm.insert(Filename(id.0.clone()), vec![count]);
                        hm
                    });
            }
        }

        index
    }

    fn merge(&mut self, index: InvertedIndex) {
        for (key, vals) in index.index {
            self.index.entry(key).or_insert_with(|| vals);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Filename(String);
struct Content(String);

/// Attempts to read all the files in a given directory and
/// send the content of each file on the channel.
///
/// Exits if a file cannot be read. If all operations are successful,
/// returns a tuple - a Receiver and JoinHandle.
fn file_reader(
    files: Vec<PathBuf>,
) -> (
    Receiver<(Filename, Content)>,
    JoinHandle<std::io::Result<()>>,
) {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        for file in files {
            let filename = if let Some(filename) = file.file_name() {
                filename.to_string_lossy()
            } else {
                Cow::from("")
            }
            .into_owned();

            let content = (Filename(filename), Content(read_to_string(file)?));

            let Ok(()) = tx.send(content) else {
                break;
            };
        }

        Ok(())
    });

    (rx, handle)
}

/// Attempts to create an inverted index from a value passed via a channel.
fn file_indexing(
    content: Receiver<(Filename, Content)>,
) -> (Receiver<InvertedIndex>, JoinHandle<()>) {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        // Receivers are iterable, so we can iterate over each.
        for (filename, content) in content.into_iter() {
            let index = InvertedIndex::from_value(filename, content);

            let Ok(_) = tx.send(index) else {
                break;
            };
        }
    });

    (rx, handle)
}

fn merge_indices(indices: Receiver<InvertedIndex>) -> (Receiver<InvertedIndex>, JoinHandle<()>) {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        let mut indices_merged = InvertedIndex::default();

        for index in indices.into_iter() {
            indices_merged.merge(index);
        }

        println!("INDEX {:?}", indices_merged);
    });

    (rx, handle)
}

fn execute(files: Vec<PathBuf>) -> io::Result<()> {
    let (reader, reader_join) = file_reader(files);
    let (indexer, indexer_join) = file_indexing(reader);
    let (merger, merger_join) = merge_indices(indexer);

    let reader_1 = reader_join.join().unwrap();
    indexer_join.join().unwrap();
    merger_join.join().unwrap();

    reader_1?;

    Ok(())
}

fn main() {
    let files_in_dir: Vec<PathBuf> = read_dir("./files")
        .unwrap()
        .filter_map(|dir| dir.ok())
        .map(|entry| entry.path())
        .collect();

    let a = execute(files_in_dir);
}
