use std::ptr::NonNull;

use crate::{chunk::Chunk, linked_list::{LinkedList, LinkedListNode}};

#[derive(Debug)]
pub struct Region {
    pub chunks: LinkedList<Chunk>,
    pub length: usize,
}

impl Region {
    pub unsafe fn first_chunk(&self) -> NonNull<LinkedListNode<Chunk>> {
        println!("CHUNKS {:?}", self);
        self.chunks.head.unwrap()
    }
}
