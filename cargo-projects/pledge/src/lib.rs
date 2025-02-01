//! This is very simple (and so far not quite efficient) memory allocator.
//! It maps entries pages for every allocation.
//! There are many ways to make it faster, examples:
//! TODO

use core::cmp::max;
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
    // Returning raw unsafe pointer
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Helps to ensure that raw pointer is correctly aligned to a specified alignment boundary.
        //
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
        let address = libc::mmap(
            ptr::null_mut(),
            aligned_layout.size(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );

        address as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Ok(aligned) = layout.align_to(max(layout.align(), *PAGE_SIZE)) {
            libc::munmap(ptr as _, aligned.pad_to_align().size());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
