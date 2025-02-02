//! This is very simple (and so far not quite efficient) memory allocator.
//! It maps entries pages for every allocation.
//! There are many ways to make it faster, examples:
//! TODO

use core::cmp::max;
use libc::MAP_FAILED;
use std::{
    alloc::{GlobalAlloc, Layout},
    ptr,
    sync::LazyLock,
};

// Unix requires to call a function to get page size
// hence initialized lazily (when accessed) once
static PAGE_SIZE: LazyLock<usize> = LazyLock::new(page_size);

// We need to get the OS page size in order to create
fn page_size() -> usize {
    // Get the size of page. A page is a contiguous block of memory
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

#[derive(Default)]
pub struct PageAllocator {}

unsafe impl GlobalAlloc for PageAllocator {
    // Layout - describes a layout of memory.
    // Returning raw unsafe pointer.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
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
        let aligned_layout = match layout.align_to(max(layout.align(), *PAGE_SIZE)) {
            Ok(l) => l.pad_to_align(),
            Err(_) => return ptr::null_mut(),
        };

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
        //
        let address = libc::mmap(
            ptr::null_mut(),
            aligned_layout.size(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );

        if address == MAP_FAILED {
            panic!("Memory mapping failed.");
        } else {
            // valid pointer
            address as _
        }
    }

    // Deallocates memory by taking in a pointer to the memory block and the size of it.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Ok(aligned) = layout.align_to(max(layout.align(), *PAGE_SIZE)) {
            // munmap() - unmaps pages of memory.
            // The function takes in an address (pointer) from where to start unmapping from and len bytes.
            // It unmaps the address + len bytes.
            let result = libc::munmap(ptr as _, aligned.pad_to_align().size());

            if result != 0 {
                panic!("Memory deallocation failed");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
