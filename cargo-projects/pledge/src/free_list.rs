use std::ptr::NonNull;

use crate::{chunk::Chunk, linked_list::{LinkedList, LinkedListNode}, Ptr};

// List type aliases
/// FreeList type aliases.
/// Free list keeps track of the free memory chunks.
pub type FreeList = LinkedList<Chunk>;
pub type FreeListNode = LinkedListNode<Chunk>;

impl FreeList {
    pub unsafe fn find_free_chunk(&self, size: usize) -> Ptr<LinkedListNode<Chunk>> {
        self.iter().find(|node| node.as_ref().data.size >= size)
    }

    unsafe fn first_from_list(&self) -> NonNull<LinkedListNode<Chunk>> {
        self.head().unwrap()
    }
}

