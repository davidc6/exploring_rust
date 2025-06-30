use crate::{content::Content, filename::Filename};
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};

/// InvertedIndex is a data structure that stores words
/// and their positions in text. Additionally, word count
/// is recorded too.
///
/// This structure is:
///
/// { index: { "some_word": { "file_one.txt": [1, 3, 5] } } }
#[derive(Default, Debug)]
pub struct InvertedIndex {
    index: HashMap<String, HashMap<Filename, Vec<usize>>>,
    word_count: usize,
}

impl InvertedIndex {
    pub fn from_value(id: Filename, value: Content) -> Self {
        let mut index = InvertedIndex::default();

        let lines = value.lines();
        let special_chars = ['.', ',', '!', '?', '"', '\''];

        for line in lines {
            let split = line.split(' ');

            for word in split {
                index.word_count += 1;

                let word = word
                    .replace(|c: char| special_chars.contains(&c), "")
                    .to_ascii_lowercase();

                index
                    .index
                    .entry(word.to_owned())
                    .and_modify(|v| v.get_mut(&id).unwrap().push(index.word_count))
                    .or_insert_with(|| {
                        let mut hm = HashMap::new();
                        hm.insert(Filename(id.to_string()), vec![index.word_count]);
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
        self.word_count += index.word_count;
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
