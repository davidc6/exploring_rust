trait PlatformT {
    unsafe fn page_size() -> usize; 
}   

pub(crate) struct Platform;

/// Get the OS page size in order to create.
///
/// A page is a contiguous block of memory.
///
/// Currently only supports Unix-like systems.
#[inline]
pub(crate) fn page_size() -> usize {
    unsafe {
        Platform::page_size()
    }
}

#[cfg(not(miri))]
#[cfg(unix)]
mod unix {
    use super::{Platform, PlatformT};

    impl PlatformT for Platform {
        unsafe fn page_size() -> usize {
            libc::sysconf(libc::_SC_PAGESIZE) as usize
        }
    }
}

#[cfg(miri)]
mod miri {
    use super::{page_size, Platform, PlatformT};

    impl PlatformT for Platform {
        unsafe fn page_size() -> usize {
            4096
        }
    }
}

