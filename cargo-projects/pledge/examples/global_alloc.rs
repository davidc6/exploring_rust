use pledge::PageAllocator;

#[global_allocator]
static ALLOC: PageAllocator = PageAllocator::lets_go_default();

fn main() {
    // Allocate a value on the heap
    let thirteen_on_heap: Box<usize> = Box::new(13);

    // &*one_on_heap - reference (&), dereference (*) a value
    // In Rust land, it is an explicit re-borrow,
    // & - references a value that is dereferenced
    // *const usize - is a pointer to a constant value of type usize
    println!("Address is: {:?}", &*thirteen_on_heap as *const usize);

    let mut vec = Vec::with_capacity(*thirteen_on_heap);

    for i in 0..*thirteen_on_heap {
        vec.push(i);
    }

    println!("Vec {vec:?} at {:?}", vec.as_ptr());
}
