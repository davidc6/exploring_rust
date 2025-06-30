use crate::index::InvertedIndex;
use std::{
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};

pub fn merge_indices(
    indices: Receiver<InvertedIndex>,
) -> (Receiver<InvertedIndex>, JoinHandle<()>) {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        let mut indices_merged = InvertedIndex::default();

        for index in indices.into_iter() {
            indices_merged.merge(index);
        }

        if !indices_merged.is_empty() {
            let Ok(_) = tx.send(indices_merged) else {
                panic!("Could not send indices");
            };
        }
    });

    (rx, handle)
}
