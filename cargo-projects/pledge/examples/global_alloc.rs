use pledge::PageAllocator;

#[global_allocator]
static ALLOC: PageAllocator = PageAllocator::lets_go_default();

fn main() {
    // Allocate a value on the heap
    let a = Box::new(1);
    println!("Result is: {:?}", a);
}
