//! This is very simple (and so far not quite efficient) memory allocator.
//! It maps entries pages for every allocation.
//! There are many ways to make it faster, examples:
//! TODO

use allocator_api2::alloc::AllocError;
use core::cmp::max;
use libc::{user, MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::{
    alloc::{GlobalAlloc, Layout},
    fmt::Pointer,
    os::raw::c_void,
    ptr::{self, slice_from_raw_parts, NonNull},
    sync::{LazyLock, Mutex},
};

mod block;

// Unix requires to call a function to get page size
// hence initialized lazily (when accessed) once
static PAGE_SIZE: LazyLock<usize> = LazyLock::new(page_size);

// We need to get the OS page size in order to create
fn page_size() -> usize {
    // Get the size of page. A page is a contiguous block of memory
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

// NonNull here is not allowed to be a null pointer essentially.
// NonNull does not guarantee that the memory being pointed to is valid however,
// or that properly aligned.
type Ptr<T> = Option<NonNull<T>>;

struct Header {
    size: usize,
    magic: usize,
}

#[derive(Debug)]
struct LinkedListNode<T> {
    prev: Ptr<Self>,
    next: Ptr<Self>,
    data: T,
    size: usize,
}

struct LinkedList<T> {
    head: Ptr<LinkedListNode<T>>,
    tail: Ptr<LinkedListNode<T>>,
    length: usize,
}

/// Chunk is where data is written to
struct Chunk {
    /// Size of the chunk in bytes
    size: usize,
    /// Is this block free and can it be used
    is_free: bool,
}

struct ChunkIter<T> {
    current: Ptr<LinkedListNode<T>>,
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn head(&self) -> Ptr<LinkedListNode<T>> {
        self.head
    }

    fn tail(&self) -> Ptr<LinkedListNode<T>> {
        self.tail
    }

    unsafe fn append(
        &mut self,
        data: T,
        size: usize,
        addr: NonNull<u8>,
    ) -> NonNull<LinkedListNode<T>> {
        // Since a (pointer) address is being passed in,
        // we need to cast to a pointer of LinkedListNode type
        // in order to then carry out operations on it.
        let node = addr.cast::<LinkedListNode<T>>();

        // Write to a memory location,
        // overriding the existing value.
        node.as_ptr().write(LinkedListNode {
            prev: self.tail,
            next: None,
            data,
            size,
        });

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
        node
    }

    unsafe fn remove(&mut self, node: NonNull<LinkedListNode<T>>) {
        let mut next = node.as_ref().next.unwrap();
        let mut prev = node.as_ref().prev.unwrap();

        prev.as_mut().next = Some(next);
        next.as_mut().prev = Some(prev);

        self.length -= 1;
    }

    fn iter(&self) -> ChunkIter<T> {
        ChunkIter { current: self.head }
    }
}

impl<T> Iterator for ChunkIter<T> {
    type Item = NonNull<LinkedListNode<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.current.map(|node| unsafe {
            self.current = node.as_ref().next;

            node
        });

        a
    }
}

type FreeList = LinkedList<Chunk>;
type FreeListNode = LinkedListNode<Chunk>;

impl FreeList {
    unsafe fn find_free_chunk(&self, size: usize) -> Ptr<Chunk> {
        self.iter()
            .find(|node| node.as_ref().size >= size)
            .map(|node| node.cast::<Chunk>())
    }
}

pub struct PageAllocator<const N: usize = 3> {
    slots: Mutex<List>,
    // size: usize,
    free_space: FreeList,
}

type List = LinkedList<()>;
type ListNode = LinkedListNode<()>;

unsafe impl<const N: usize> Sync for PageAllocator<N> {}

impl PageAllocator {
    pub const fn default_config() -> Self {
        Self {
            slots: Mutex::new(LinkedList::new()),
            // size: 0,
            free_space: FreeList::new(),
        }
    }

    // Return an address which then can be casted to a pointer
    unsafe fn allocate(&self, layout: Layout) -> NonNull<[u8]> {
        let size = layout.size();

        // check if free block exists that will be enough
        // let arena = self.free_space.into_iter().find(|x| x.
        let chunk = match self.free_space.find_free_chunk(size) {
            Some(val) => val,
            None => NonNull::new_unchecked(libc::mmap(
                ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            ))
            .cast(),
        };

        // if chunk.is_none() {
        //     println!("NONE");
        // }

        // println!("Actual size {:?}", size);
        // TODO: find a free block, check if possible to get a block

        // TODO
        // 1. need to check if the memory allocator actually has available memory ot not
        // 2. request memory from OS
        // let fd = -1;
        // let addr = libc::mmap(
        //     ptr::null_mut(),
        //     size,
        //     PROT_READ | PROT_WRITE,
        //     MAP_PRIVATE | MAP_ANONYMOUS,
        //     fd,
        //     0,
        // );
        // let addr = NonNull::new_unchecked(addr).cast();

        // TODO: This essentially writes to the address above which messes up the original address
        // let a = match self.slots.lock() {
        //     Ok(mut list) => Ok(list.append((), size, addr)),
        //     Err(_) => Err(AllocError),
        // };

        // let a = a.unwrap();
        // let content_addr = NonNull::new_unchecked(a.as_ptr()).cast();
        // let size = a.as_ref().size;

        NonNull::slice_from_raw_parts(chunk.cast(), size)
    }

    unsafe fn deallocate(&self, ptr: *mut u8, layout: Layout) {
        if libc::munmap(ptr as _, layout.size()) != 0 {
            // handle issues here
        }
    }
}

unsafe impl GlobalAlloc for PageAllocator {
    // Layout - describes a layout of memory.
    // Returning raw unsafe pointer which is the address of the allocated memory.
    // Specifically the beginning of the memory block allocated.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout).cast().as_ptr()

        // Check whether a specific layout (description of memory size and alignment of a type)
        // can be aligned to a desired alignment.

        // Alignment is necessary for faster memory access since CPUs read in chunks,
        // misaligned data can cause slower reads.
        // Bad memory layout (bad memory ordering, inefficient usage etc) leads to wasted space
        // and poor performance.

        // align_to() does not add any padding to the overall size
        // and will fail if it's less strict than the original alignment

        // max - we look at either current layout minimum alignment or OS specific.
        // If layout fais to align, we return a null mutable pointer (which has the address 0).
        // let aligned_layout = match layout.align_to(max(layout.align(), *PAGE_SIZE)) {
        //     Ok(l) => l.pad_to_align(),
        //     Err(_) => return ptr::null_mut(),
        // };

        // for Unix-like systems only
        // mmap - creates a new mapping in the
        // virtual address of the calling process.
        // We pass:
        //   1) null mutable raw pointer (zero initializing a pointer, the resulting address is 0.
        //   2) minimum size for the memory block of this size (in bytes)
        //   3) read and write flags to be able to read and write to
        //   4) the memory is private to process and does not represent a file stored in memory.
        //   5) not a file in memory
        //
        // We get mutable raw pointer to an unsized, untyped block of memory.
        //
        // *mut   - mutable raw pointer that does not have any safety guarantees
        // void_c - equivalent to C void, when the type of data is not specified.
        //

        // Memory is protected for read + write only.
        // let memory_protection = PROT_READ | PROT_WRITE;
        // Make memory private to our process.
        // let flags = MAP_PRIVATE | MAP_ANONYMOUS;
        // From the man: some implementations require fd to be -1 if MAP_ANONYMOUS
        // (or MAP_ANON) is specified, and portable applications
        // should ensure this.
        // let fd = -1;
        // let address = libc::mmap(
        //     ptr::null_mut(),
        //     aligned_layout.size(),
        //     memory_protection,
        //     flags,
        //     fd,
        //     0,
        // );

        // TODO: We need a better way to handle the error here ie. an Option.
        // if address == MAP_FAILED {
        //     panic!("Memory mapping failed.");
        // } else {
        // valid pointer
        // address as _
        // }
    }

    // Deallocates memory by taking in a pointer to the memory block and the size of it.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // if let Ok(aligned) = layout.align_to(max(layout.align(), *PAGE_SIZE)) {
        //     // munmap() - unmaps pages of memory.
        //     // The function takes in an address (pointer) from where to start unmapping from and len bytes.
        //     // It unmaps the address + len bytes.
        //     let result = libc::munmap(ptr as _, aligned.pad_to_align().size());

        //     if result != 0 {
        //         // TODO: Is there a better way to handle this?
        //         panic!("Memory deallocation failed");
        //     }
        // }

        // if let Ok(mut alloc) = self.slots.lock() {

        self.deallocate(ptr, layout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocator_wrapper_works() {
        let allocator = PageAllocator::default_config();

        unsafe {
            // Initial allocation
            // let layout = Layout::array::<u8>(8).unwrap();
            let layout = Layout::new::<[u8; 20]>();
            let a = layout.align_to(8).unwrap();
            let allocated = allocator.allocate(a).as_mut();

            // fill with values
            allocated.fill(10);

            // Second allocation
            // let layout_another = Layout::array::<u8>(4096).unwrap();
            let layout_another = Layout::new::<[u8; 23]>();
            let b = layout_another.align_to(8).unwrap();
            let allocated_2 = allocator.allocate(b).as_mut();

            // fill with values
            allocated_2.fill(13);

            for value in allocated.iter() {
                assert!(value == &10);
            }

            allocator.deallocate(allocated.as_mut_ptr(), layout);

            for value in allocated_2.iter() {
                assert!(value == &13);
            }

            allocator.deallocate(allocated_2.as_mut_ptr(), layout_another);
        }
    }
}
