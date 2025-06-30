use crate::index::InvertedIndex;
use std::{
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};

pub fn write_indices(
    indices: Receiver<InvertedIndex>,
) -> (Receiver<InvertedIndex>, JoinHandle<()>) {
    let (tx, rx) = channel();

    let handler = thread::spawn(|| {
        for index in indices.into_iter() {
            // TODO
            println!("{:?}", index);
        }
    });

    (rx, handler)
}
