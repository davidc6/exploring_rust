use pledge::PageAllocator;

#[global_allocator]
static ALLOC: PageAllocator = PageAllocator::default_config();

fn main() {
    // Allocate a value on the heap
    let thirteen_on_heap: Box<usize> = Box::new(13);

    // &*one_on_heap - reference (&), dereference (*) a value
    // In Rust land, it is an explicit re-borrow,
    // & - references a value that is dereferenced
    // *const usize - is a pointer to a constant value of type usize
    println!("Address (box) is: {:?}", &*thirteen_on_heap as *const usize);

    let mut vec = Vec::with_capacity(*thirteen_on_heap);

    for i in 0..*thirteen_on_heap {
        vec.push(i);
    }

    println!("Address (vec) is: {:?}", vec.as_ptr());

    let four_on_heap: Box<usize> = Box::new(4);

    println!("Address (box 2) is: {:?}", &*four_on_heap as *const usize);
}
