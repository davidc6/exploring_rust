use std::ptr::NonNull;
use crate::{linked_list::LinkedListNode, Ptr};

/// Chunk is where data is written to
#[derive(Debug, Clone, Copy)]
pub struct Chunk {
    /// Size of the chunk in bytes
    pub size: usize,
    /// Is this block free and can it be used
    pub is_free: bool,
}

pub struct ChunkIter<T> {
    pub current: Ptr<LinkedListNode<T>>,
}

impl<T> Iterator for ChunkIter<T> {
    type Item = NonNull<LinkedListNode<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            self.current = node.as_ref().next;

            node
        })
    }
}

