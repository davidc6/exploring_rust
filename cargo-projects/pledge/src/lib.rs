use core::cmp::max;
use std::{
    alloc::{GlobalAlloc, Layout},
    ptr,
    sync::LazyLock,
};

// Unix requires to call a function to get page size
// hence initialized lazily (when accessed) once
static PAGE_SIZE: LazyLock<usize> = LazyLock::new(page_size);

fn page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

// unsafe impl GlobalAlloc for

// Returning raw unsafe pointer here
unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    // TODO
    let aligned_layout = match layout.align_to(max(layout.align(), *PAGE_SIZE)) {
        Ok(l) => l.pad_to_align(),
        Err(_) => return ptr::null_mut(),
    };

    // for Unix-like systems only
    // mmap - creates a new mapping in the
    // virtual address of the calling process
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
