use std::sync::atomic::fence;
use std::usize;
use std::{ptr::NonNull, sync::atomic::AtomicUsize};
use std::ops::Deref;

/// Count the number of Arc objects that share an allocation.
///
/// The struct holds the counter and object of type T.
///
/// This is an internal implementation detail of Arc implementation.
struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T
}

/// Arc<T> is a pointer to a (shared) ArcData<T> object.
/// Box represents 

/// Instead of using a Box to handle allocations of ArcData<T>,
/// we use a pointer. We handle allocations and ownership manually.
/// NonNull represents a pointer that is never null. 
///
/// The compiler assumes that T is never Sync or Send unless 
/// we tell it otherwise.
pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>
}

/// If/when Arc<T> is sent across threads, T needs to be Sync. Since 
/// a thread could drop T, it needs to be Send.
///
/// Arc<T> should be Send if and if T is Sync + Send. The same 
/// applies to Sync since shared &Arc<T> can be cloned into a new Arc<T>.
///
/// Sync - safe to share references between threads.
/// Send - move ownership of T from one thread to another.
unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

/// let a = Arc::new(123)
///
/// Create a new allocation with a reference count set to 1.
/// Box::new creates a new allocation and Box::leak gives up exclusive 
/// ownership of the allocation. NonNull turns it into a pointer.
impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(
                ArcData {
                    ref_count: AtomicUsize::new(1),
                    data,
                }
            ))),
        }
    }

    /// Pointer will point to a valid ArcData<T> as long as the object exists.
    /// The compiler does not know this however, therefore accessing data 
    /// through the pointer requires unsafe code. 
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

/// Using Deref, we can make Arc<T> behave like a reference to T.
///
/// DerefMut is not implemented since Arc<T> represents shared ownership and cannot be 
/// mutated hence T cannot be &mut T. 
///
/// let arc = Arc::new(1);
/// let arc_derefed = *arc;
///
/// What ran behind the scenes - *(y.deref())
impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

// For cloning, the same pointer is used and the reference number is incremented.
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // fetch_add() - fetches the current count and increments by the value (1 in this case)
        // We don't want to overflow when cloning so to avoid that, we have a usize check
        if self.data().ref_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }

        Arc {
            ptr: self.ptr
        }
    }
}

// Decrement the ref count by dropping an Arc
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // Memory ordering cannot be "Relaxed" since we need to make sure that nothing else is
        // accessing the data when we drop it.
        if self.data().ref_count.fetch_sub(1, std::sync::atomic::Ordering::Release) == 1 {
            // Use "Acquire" for the last drop/last decrement.
            // All memory operations after the fence can see operations before it.
            fence(std::sync::atomic::Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

#[test]
fn test_drop_count() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DropDetect;

    impl Drop for DropDetect {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    let a_1 = Arc::new(("hi", DropDetect));
    let a_2 = a_1.clone();

    let t = std::thread::spawn(move || {
        assert_eq!(a_1.0, "hi");
    });

    t.join().unwrap();

    assert_eq!(NUM_DROPS.load(std::sync::atomic::Ordering::Relaxed), 0);

    drop(a_2);

    assert_eq!(NUM_DROPS.load(std::sync::atomic::Ordering::Relaxed), 1);
}

