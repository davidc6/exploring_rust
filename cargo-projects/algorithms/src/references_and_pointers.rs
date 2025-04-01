/// & - a reference in Rust is a basic pointer type, single machine word bolding the address of the referrer.
/// In Rust (&) means borrowing a reference to x.
/// 
/// Both & and * hold an address for some memory.
/// 
/// * - we cannot dereference a raw pointer without the unsafe keyword.
/// 
/// References are never null in (safe) Rust.
/// 
/// Box - is the simplest way to allocate a value on a heap (it's an owning pointer when the owner is dropped
/// the referent is dropped also). Non-owning types are references.
/// 
/// References are created explicitly with & and dereferences with * operators.
/// 
/// If a function takes a reference we don't have to deref to use . operator, it is automatically
/// done for us. This is because references are widely used in Rust.
/// 
/// e.g. 
///     struct Film { title: &'static str, year: u8 };
///     let film = Film { title: "Some title", year: 2024 };
///     let film_ref = &film;
///     let title = film_ref.title; (same as let tile = *film_ref.title)
/// 
/// & and * operators in Rust are used to create and follow references. 
/// . operator borrows and dereferences implicitly. Same goes for comparing references.
/// 
/// References are simple addresses. There are also two kinds of pointes - fat pointer (carrying an address and 
/// additional metadata).
///     - A references to a slice is a fat pointer (pointer + length)
///     - Other kind of fat pointer is a trait object, a reference to a value that implements a trait
/// 
/// Raw pointers - *mut T and *const T 