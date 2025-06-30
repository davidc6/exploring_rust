use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};

use crate::{content::Content, filename::Filename};

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
pub struct InvertedIndex {
    index: HashMap<String, HashMap<Filename, Vec<usize>>>,
}

impl InvertedIndex {
    pub fn from_value(id: Filename, value: Content) -> Self {
        let mut index = InvertedIndex::default();

        let lines = value.lines();
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
                        hm.insert(Filename(id.to_string()), vec![count]);
                        hm
                    });
            }
        }

        index
    }

    pub fn merge(&mut self, index: InvertedIndex) {
        for (key, vals) in index.index {
            self.index.entry(key).or_insert_with(|| vals);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.index.len() == 0
    }
}

/// Attempts to create an inverted index from a value passed via a channel.
pub fn file_indexing(
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
