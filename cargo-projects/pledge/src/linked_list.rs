use std::ptr::NonNull;
use crate::{chunk::{Chunk, ChunkIter}, free_list::FreeListNode, Ptr};

type List = LinkedList<()>;
type ListNode = LinkedListNode<()>;

#[derive(Debug)]
pub struct LinkedListNode<T> {
    pub prev: Ptr<Self>,
    pub next: Ptr<Self>,
    pub data: T,
    pub size: usize,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn head(&self) -> Ptr<LinkedListNode<T>> {
        self.head
    }

    fn tail(&self) -> Ptr<LinkedListNode<T>> {
        self.tail
    }

    pub unsafe fn append(&mut self, data: T, size: usize, addr: NonNull<u8>) -> Ptr<LinkedListNode<T>> {
        // Since a (pointer) address is being passed in,
        // we need to cast to a pointer of LinkedListNode type
        // in order to then carry out operations on it.
        let mut node = addr.cast::<LinkedListNode<T>>();

        // Write to a memory location,
        // overriding the existing value.
        node.as_ptr().write(LinkedListNode {
            prev: None,
            next: None,
            data,
            size,
        });

        // We don't want to have to set the previous value if there not nodes yet
        if self.length > 0 {
            node.as_mut().prev = self.tail;
        }

        // If there a tai node, we want to add append (.next) a new node to it
        if let Some(mut tail) = self.tail {
            tail.as_mut().next = Some(node);
        } else {
            // If there isn't a tail node, we set head to new node
            self.head = Some(node);
        }

        // New node is the tail now
        self.tail = Some(node);
        self.length += 1;

        // Return the newly appended node
        Some(node)
    }

    pub unsafe fn insert_after(
        &mut self,
        mut current_node: NonNull<LinkedListNode<T>>,
        data: T,
        new_node_addr: NonNull<u8>,
        size: usize,
    ) -> NonNull<LinkedListNode<T>> {
        let new_node = new_node_addr.cast::<LinkedListNode<T>>();

        // Insert new node
        new_node.as_ptr().write(LinkedListNode {
            prev: Some(current_node),
            next: current_node.as_ref().next,
            data,
            size,
        });

        if current_node == self.tail.unwrap() {
            self.tail = Some(new_node);
        } else {
            current_node.as_ref().next.unwrap().as_mut().prev = Some(new_node);
        }

        current_node.as_mut().next = Some(new_node);

        self.length += 1;

        new_node
    }

    pub unsafe fn remove(&mut self, mut node: NonNull<LinkedListNode<T>>) {
        if self.length == 1 {
            self.head = None;
            self.tail = None;
        } else if node == self.head.unwrap() {
            node.as_mut().next.unwrap().as_mut().prev = None;
            self.head = node.as_ref().next;
        } else if node == self.tail.unwrap() {
            node.as_mut().prev.unwrap().as_mut().next = None;
            self.tail = node.as_ref().prev;
        } else {
            let mut next = node.as_ref().next.unwrap();
            let mut prev = node.as_ref().prev.unwrap();
            prev.as_mut().next = Some(next);
            next.as_mut().prev = Some(prev);
        }

        self.length -= 1;
    }

    pub fn iter(&self) -> ChunkIter<T> {
        ChunkIter { current: self.head }
    }
}


#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Ptr<LinkedListNode<T>>,
    pub tail: Ptr<LinkedListNode<T>>,
    pub length: usize,
}

impl LinkedListNode<Chunk> {
    unsafe fn from_list_node(n: NonNull<FreeListNode>) -> NonNull<Self> {
        Self::from_addr(n.cast())
    }

    unsafe fn from_addr(address: NonNull<u8>) -> NonNull<Self> {
        NonNull::new_unchecked(address.as_ptr().cast::<Self>())
    }
}

