use std::{alloc::Layout, ptr::NonNull, sync::Mutex};
use allocator_api2::alloc::AllocError;

use crate::{free_list::FreeList, linked_list::LinkedList, InnerAlloc};

/// NonNull does not guarantee that the memory that is pointed to is valid.
/// It is essentially just a wrapper type that reinforces that the pointer isn't null.
/// It is not allowed to be a null therefore and must always be ensured that it's non-null.
type AllocResult = Result<NonNull<[u8]>, AllocError>;

pub struct PageAllocator<const N: usize = 3> {
    allocator: Mutex<InnerAlloc>,
}

unsafe impl<const N: usize> Sync for PageAllocator<N> {}

impl PageAllocator {
    pub const fn default_config() -> Self {
        PageAllocator {
            allocator: Mutex::new(InnerAlloc {
                free_space: FreeList::new(),
                regions: LinkedList::new(),
            }),
        }
    }

    pub unsafe fn allocate(&self, layout: Layout) -> AllocResult {
        match self.allocator.lock() {
            Ok(mut allocator) => Ok(allocator.allocate(layout)),
            Err(_) => Err(AllocError),
        }
    }

    pub unsafe fn deallocate(&self, ptr: *mut u8, layout: Layout) {
        if let Ok(mut allocator) = self.allocator.lock() {
            allocator.deallocate(ptr, layout)
        }
    }
}
