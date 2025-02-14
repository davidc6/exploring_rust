//! This is very simple (and so far not quite efficient) memory allocator.
//! It maps entries pages for every allocation.
//! There are many ways to make it faster, examples:
//! TODO

use allocator_api2::alloc::AllocError;
use core::cmp::max;
use libc::{user, MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::{
    alloc::{GlobalAlloc, Layout},
    os::raw::c_void,
    ptr::{self, NonNull},
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

struct LinkedListNode<T> {
    prev: Ptr<Self>,
    next: Ptr<Self>,
    data: T,
}

struct LinkedList<T> {
    head: Ptr<LinkedListNode<T>>,
    tail: Ptr<LinkedListNode<T>>,
    length: usize,
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

    unsafe fn append(&mut self, data: T, addr: NonNull<u8>) -> NonNull<LinkedListNode<T>> {
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
}

pub struct PageAllocator<const N: usize = 3> {
    slots: Mutex<List>,
    // size: usize,
}

type List = LinkedList<()>;
type ListNode = LinkedListNode<()>;

unsafe impl<const N: usize> Sync for PageAllocator<N> {}

impl PageAllocator {
    pub const fn lets_go_default() -> Self {
        Self {
            slots: Mutex::new(LinkedList::new()),
            // size: 0,
        }
    }

    // Return an address which then can be casted to a pointer
    unsafe fn allocate(&self, layout: Layout) -> Result<NonNull<LinkedListNode<()>>, AllocError> {
        let size = layout.size();

        // TODO: find a free block, check if possible to get a block

        // TODO
        // 1. need to check if the memory allocator actually has available memory ot not
        // 2. request memory from OS
        let fd = -1;
        let addr = libc::mmap(
            ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            fd,
            0,
        );
        let addr = NonNull::new_unchecked(addr).cast();

        match self.slots.lock() {
            Ok(mut list) => Ok(list.append((), addr)),
            Err(_) => Err(AllocError),
        }

        // let node = self.slots.head().unwrap_unchecked();

        // TODO: return an address

        // Ok(node)

        // Ok(self.slots.append())

        // self.slots
    }
}

unsafe impl GlobalAlloc for PageAllocator {
    // Layout - describes a layout of memory.
    // Returning raw unsafe pointer which is the address of the allocated memory.
    // Specifically the beginning of the memory block allocated.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let a = self.allocate(layout).unwrap();
        let a = a.cast().as_ptr();
        a

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
        if let Ok(aligned) = layout.align_to(max(layout.align(), *PAGE_SIZE)) {
            // munmap() - unmaps pages of memory.
            // The function takes in an address (pointer) from where to start unmapping from and len bytes.
            // It unmaps the address + len bytes.
            let result = libc::munmap(ptr as _, aligned.pad_to_align().size());

            if result != 0 {
                // TODO: Is there a better way to handle this?
                panic!("Memory deallocation failed");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn allocator_test<A: GlobalAlloc>(allocator: A) {
        // Create a memory layout
        let layout = Layout::new::<[i32; 100]>();

        // Allocate memory
        let ptr = allocator.alloc(layout);

        // Check that pointer is not null
        assert!(!ptr.is_null());

        // Create a slice from a pointer
        let slice = std::slice::from_raw_parts_mut(ptr as *mut i32, 100);

        // Check that we have 100 items
        assert_eq!(slice.len(), 100);

        // Give each slice item a value
        for (i, item) in slice.iter_mut().enumerate() {
            *item = i as i32;
        }

        for (i, item) in slice[0..100].iter().enumerate() {
            assert_eq!(*item, i as i32);
        }
    }

    // #[test]
    // fn it_works() {
    //     unsafe {
    //         allocator_test(PageAllocator {
    //             slots: LinkedList::new(),
    //             size: 0,
    //         });
    //     }
    // }
}
