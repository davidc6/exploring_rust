//! This is very simple (and so far not quite efficient) memory allocator.
//! It maps entries pages for every allocation.
//! There are many ways to make it faster, examples:
//! TODO

use allocator_api2::alloc::AllocError;
use libc::{MAP_ANONYMOUS, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::{
    alloc::{GlobalAlloc, Layout},
    ptr::{self, NonNull},
    sync::{LazyLock, Mutex},
};

mod block;

/// We need to get the OS page size in order to create
fn page_size() -> usize {
    // Get the size of page. A page is a contiguous block of memory
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

/// Unix requires to call a function to get the page size
/// hence initialized lazily (only when accessed) once.
static PAGE_SIZE: LazyLock<usize> = LazyLock::new(page_size);

/// NonNull does not guarantee that the memory that is pointed to is valid.
/// It is essentially just a wrapper type that reinforces that the pointer isn't null.
/// It is not allowed to be a null therefore and must always be ensured that it's non-null.
type Ptr<T> = Option<NonNull<T>>;
type AllocResult = Result<NonNull<[u8]>, AllocError>;

/// FreeList type aliases.
/// Free list keeps track of the free memory chunks.
type FreeList = LinkedList<Chunk>;
type FreeListNode = LinkedListNode<Chunk>;

// List type aliases
type List = LinkedList<()>;
type ListNode = LinkedListNode<()>;

struct Header {
    size: usize,
    magic: usize,
}

#[derive(Debug)]
pub(crate) struct LinkedListNode<T> {
    prev: Ptr<Self>,
    next: Ptr<Self>,
    data: T,
    size: usize,
}

impl LinkedListNode<Chunk> {
    unsafe fn from_list_node(n: NonNull<FreeListNode>) -> NonNull<Self> {
        Self::from_addr(n.cast())
    }

    unsafe fn from_addr(address: NonNull<u8>) -> NonNull<Self> {
        NonNull::new_unchecked(address.as_ptr().cast::<Self>())
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Ptr<LinkedListNode<T>>,
    tail: Ptr<LinkedListNode<T>>,
    length: usize,
}

/// Chunk is where data is written to
#[derive(Debug, Clone, Copy)]
struct Chunk {
    /// Size of the chunk in bytes
    size: usize,
    /// Is this block free and can it be used
    is_free: bool,
}

struct ChunkIter<T> {
    current: Ptr<LinkedListNode<T>>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
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

    unsafe fn append(&mut self, data: T, size: usize, addr: NonNull<u8>) -> Ptr<LinkedListNode<T>> {
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

    unsafe fn insert_after(
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

    unsafe fn remove(&mut self, mut node: NonNull<LinkedListNode<T>>) {
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

    fn iter(&self) -> ChunkIter<T> {
        ChunkIter { current: self.head }
    }
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

impl FreeList {
    unsafe fn find_free_chunk(&self, size: usize) -> Ptr<LinkedListNode<Chunk>> {
        self.iter().find(|node| node.as_ref().data.size >= size)
    }

    unsafe fn first_from_list(&self) -> NonNull<LinkedListNode<Chunk>> {
        self.head().unwrap()
    }
}

pub struct PageAllocator<const N: usize = 3> {
    allocator: Mutex<InnerAlloc>,
}

unsafe impl<const N: usize> Sync for PageAllocator<N> {}

pub struct InnerAlloc {
    free_space: FreeList,
}

impl InnerAlloc {
    // Return an address which then can be casted to a pointer
    unsafe fn allocate(&mut self, layout: Layout) -> NonNull<[u8]> {
        let size = layout.size();
        let free_chunk = self.free_space.find_free_chunk(size);

        // check if free block exists that will be enough.
        // TODO: "size" here has to be something meaningful and not just the size of an object to allocate
        let mut chunk = match free_chunk {
            Some(val) => val,
            None => {
                let page_size = *PAGE_SIZE;

                // Ask for memory from OS using mmap() system call.
                // This currently only works on Linux (TODO).
                // - Memory protection
                // Memory is protected, the contents of the region can be READ and modified (WRITE)
                // - Memory mapping
                // Make memory private to our process (MAP_PRIVATE | MAP_ANONYMOUS)
                // MAP_PRIVATE - other processes that map tha same file,
                // cannot see updates to the mapping.
                // MAP_ANONYMOUS - large zero-filled blocks not backed by a file
                let addr = NonNull::new_unchecked(libc::mmap(
                    ptr::null_mut(),
                    page_size,
                    PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS,
                    -1,
                    0,
                ))
                .cast();

                // Add to the list of free space chunks
                let node = self.free_space.append(
                    Chunk {
                        size: page_size,
                        is_free: true,
                    },
                    page_size,
                    addr,
                );

                node.unwrap()
            }
        };

        // TODO: can we split the memory chunk to only use what we need to?
        // i.e. we don't need 4096 bytes if we only need 128 bytes
        if chunk.as_ref().size > size {
            let chunk_size = chunk.as_ref().data.size - size;

            self.free_space.insert_after(
                chunk,
                Chunk {
                    size: chunk_size,
                    is_free: true,
                },
                NonNull::new_unchecked(chunk.as_ptr().add(size).cast()),
                chunk_size,
            );

            chunk.as_mut().size = size;
            chunk.as_mut().data.size = size;
        }

        // No longer a free chunk since it's used for allocation
        self.free_space.remove(chunk);
        chunk.as_mut().data.is_free = false;

        // TODO
        // 1. need to check if the memory allocator actually has available memory ot not
        // 2. request memory from OS
        // let addr = NonNull::new_unchecked(addr).cast();

        // TODO: This essentially writes to the address above which messes up the original address
        // let a = match self.slots.lock() {
        //     Ok(mut list) => Ok(list.append((), size, addr)),
        //     Err(_) => Err(AllocError),
        // };

        let w = chunk.cast();

        NonNull::slice_from_raw_parts(w, size)
    }

    unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        if libc::munmap(ptr as _, layout.size()) != 0 {
            // TOOD: How should we handle issues here?
        }
    }
}

impl PageAllocator {
    pub const fn default_config() -> Self {
        PageAllocator {
            allocator: Mutex::new(InnerAlloc {
                free_space: FreeList::new(),
            }),
        }
    }

    unsafe fn allocate(&self, layout: Layout) -> AllocResult {
        match self.allocator.lock() {
            Ok(mut allocator) => Ok(allocator.allocate(layout)),
            Err(_) => Err(AllocError),
        }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, layout: Layout) {
        if let Ok(mut allocator) = self.allocator.lock() {
            allocator.deallocate(ptr, layout)
        }
    }
}

unsafe impl GlobalAlloc for PageAllocator {
    // Layout - describes a layout of memory.
    // Returning raw unsafe pointer which is the address of the allocated memory.
    // Specifically the beginning of the memory block allocated.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout).unwrap().cast().as_ptr()

        // Check whether a specific layout (description of memory size and alignment of a type)
        // can be aligned to a desired alignment.

        // Alignment is necessary for faster memory access since CPUs read in chunks,
        // misaligned data can cause slower reads.
        // Bad memory layout (bad memory ordering, inefficient usage etc) leads to wasted space
        // and poor performance.

        // align_to() does not add any padding to the overall size
        // and will fail if it's less strict than the original alignment

        // max - we look at either current layout minimum alignment or OS specific.
        // If layout fails to align, we return a null mutable pointer (which has the address 0).
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

        // let memory_protection = PROT_READ | PROT_WRITE;
        // From the man: some implementations require fd to be -1 if MAP_ANONYMOUS
        // (or MAP_ANON) is specified, and portable applications
        // should ensure this.

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
            let layout = Layout::new::<[u8; 8]>();
            let mut allocated = allocator.allocate(layout).unwrap();

            // Fill with values
            allocated.as_mut().fill(10);

            // Second allocation
            let layout_another = Layout::new::<[u8; 16]>();
            let mut allocated_2 = allocator.allocate(layout_another).unwrap();

            // Fill with values
            allocated_2.as_mut().fill(13);

            for value in allocated.as_ref() {
                assert!(value == &10);
            }

            allocator.deallocate(allocated_2.cast().as_ptr(), layout);

            for value in allocated_2.as_ref() {
                assert!(value == &13);
            }

            allocator.deallocate(allocated_2.cast().as_ptr(), layout_another);
        }
    }
}
