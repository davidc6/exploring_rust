//! This is very simple (and so far not quite efficient) memory allocator.
//! It maps entries pages for every allocation.
//! There are many ways to make it faster, examples:
//! TODO

use allocator_api2::alloc::AllocError;
use libc::{MAP_ANONYMOUS, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::{
    alloc::{GlobalAlloc, Layout}, ptr::{self, NonNull}, sync::{LazyLock, Mutex}
};
use platform::page_size;
use free_list::FreeList;

use crate::{chunk::Chunk, linked_list::LinkedList, region::Region};

mod block;
mod platform;
mod free_list;
mod linked_list;
mod chunk;
mod region;

/// NonNull does not guarantee that the memory that is pointed to is valid.
/// It is essentially just a wrapper type that reinforces that the pointer isn't null.
/// It is not allowed to be a null therefore and must always be ensured that it's non-null.
type AllocResult = Result<NonNull<[u8]>, AllocError>;

type Ptr<T> = Option<NonNull<T>>;

/// Unix requires to call a function to get the page size
/// hence initialized lazily (only when accessed) once.
static PAGE_SIZE: LazyLock<usize> = LazyLock::new(page_size);

// List type aliases

struct Header {
    size: usize,
    magic: usize,
}

pub struct PageAllocator<const N: usize = 3> {
    allocator: Mutex<InnerAlloc>,
}

unsafe impl<const N: usize> Sync for PageAllocator<N> {}

pub struct InnerAlloc {
    free_space: FreeList,
    regions: LinkedList<Region>,
}

impl InnerAlloc {
    /// Return an address which then can be casted to a pointer.
    unsafe fn allocate(&mut self, layout: Layout) -> NonNull<[u8]> {
        // How many bytes does the allocation need?
        let size = layout.size();
        // Find a free chunk in the free space.
        let free_chunk = self.free_space.find_free_chunk(size);

        // check if free block exists that will be enough.
        // TODO: 
        //  - "size" here has to be meaningful, not just the size of an object to allocate.
        let mut chunk = match free_chunk {
            Some(val) => val,
            None => {
                let page_size = *PAGE_SIZE;

                // Ask for memory from OS using mmap() system call.
                // TODO: This currently only works on Linux.
                // - Memory protection
                // Memory is protected, the contents of the region can be READ and modified (WRITE)
                // - Memory mapping
                // Make memory private to our process (MAP_PRIVATE | MAP_ANONYMOUS)
                // MAP_PRIVATE - other processes that map the same file,
                // cannot see updates to the mapping.
                // MAP_ANONYMOUS - large zero-filled blocks not backed by a file.
                // From the man: some implementations require fd to be -1 if MAP_ANONYMOUS
                // (or MAP_ANON) is specified, and portable applications
                // should ensure this.
                let addr = NonNull::new_unchecked(libc::mmap(
                    ptr::null_mut(),
                    page_size,
                    PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS,
                    -1,
                    0,
                ))
                .cast();

                let region = self.regions.append(
                    Region {
                        chunks: LinkedList::new(),
                        length: page_size,
                    },
                    page_size,
                    addr,
                );

                let chunk = region.unwrap().as_mut().data.chunks.append(
                    Chunk {
                        size: region.unwrap().as_ref().size,
                        is_free: true,
                    },
                    region.unwrap().as_ref().size,
                    addr,
                );

                println!("DODO {:?}", chunk.unwrap().as_ref().data);
                println!(
                    "DODO {:?}",
                    region.unwrap().as_ref().data.first_chunk().as_ref().data
                );

                // Add to the list of free space chunks
                self.free_space.append(
                    Chunk {
                        size: region.unwrap().as_ref().size,
                        is_free: true,
                    },
                    region.unwrap().as_ref().size,
                    addr,
                );

                // self.free_space.append(data, size, addr)

                // let a = &region.unwrap().as_ref();

                // println!("ERRRRRRO {:?}", a.data);
                // println!("ERRRRRRO {:?}", chunk.as_ref().unwrap().as_ref().data);A

                println!("ERRRROR {:?}", region.unwrap().as_ref());

                // self.regions.

                // chunk.unwrap()
                region.unwrap().as_ref().data.first_chunk()

                // chunk.unwrap()
                // node.unwrap(
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
        // TODO
        // 1. get the chunk from the pointer
        // let chunk = NonNull::new_unchecked(ptr).cast::<LinkedListNode<Chunk>>();

        if self.free_space.length != 0 {
            return;
        }

        if libc::munmap(ptr as _, layout.size()) != 0 {
            // TODO: How should we handle issues here?
        }

        // 2. can we merge surrounding block?

        // 3. append to free chunks list
    }
}

impl PageAllocator {
    pub const fn default_config() -> Self {
        PageAllocator {
            allocator: Mutex::new(InnerAlloc {
                free_space: FreeList::new(),
                regions: LinkedList::new(),
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

/// Registers as the standard library default allocator.
unsafe impl GlobalAlloc for PageAllocator {
    /// Layout - describes a layout of memory (i.e. size in bytes and alignment for allocation).
    ///
    /// Returning raw unsafe pointer which is the address of the allocated memory.
    /// Specifically the beginning of the memory block allocated.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self.allocate(layout) {
            Ok(addr) => addr.cast().as_ptr(),
            Err(_) => ptr::null_mut(),
        }

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

            // TODO: Re-enable once figures the strategy
            // allocator.deallocate(allocated.cast().as_ptr(), layout);

            for value in allocated_2.as_ref() {
                assert!(value == &13);
            }

            allocator.deallocate(allocated_2.cast().as_ptr(), layout_another);
        }
    }
}
