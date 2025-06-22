# Interior Mutability

- [Cell](#cell)
- [RefCell](#refcell)
- [UnsafeCell](#unsafecell)

Some types provide a way to mutate the value through a shared reference. Rust
usually checks references at compile time but here the checks are carried out at
run-time. If the references rules are broken then you'll get a panic. There type
usually rely on mechanisms such as:

- Atomic CPU instruction
- Invariant -  

to provide safe mutability without using an exclusive reference. There are
usually two types of such types:

1. Types that give out a mutable reference through a shared reference - Mutex or
RefCell - these types ensure that for any value the mutable reference is given
to, only one mutable reference can exist at a time. These types rely on
UnsafeCell under the hood.  

2. Types that allow to replace the value given only
a shared reference - These gives methods to manipulate the underlying/inner data
in place. These are types like std::sync::atomic, std::cell::Cell. No reference
is given to types such as usize or i32 but they can be replaced.

Interior mutability is useful when we need a mutate a bit of data inside of the
otherwise immutable value. As mentioned there are several types that provide
these. The most straightforward ones are `Cell<T>` and `RefCell<T>`. `Cell<T>`
contains a single value it provides get and set functionality on it without the
mut access.

## Cell<T>

```rs
let cell_value = Cell::new(1);

// Returns 1, a copy actually and only work if T implements Copy so the type has
// to be Copyable
cell_value.get();

// Sets to 2
cell_value.set(2);
```

Cells are designed to bend the rules of immutability.

## RefCell<T>

Similar to Cell, RefCell is a generic type that contains a single value. RefCell
supports borrowing references to the T value. It does not implement `Send` and `Sync` and is designed for a single single-thread use. 

```rs 
let refcell_value = RefCell::new(1);

// Borrows the value, it's a shared reference to the stored value.  This will
// panic if there's already an mutable borrow before this.  Return type is
// Ref<T>
refcell_value.borrow();

// Borrows mutably.  This will panic if already borrow.  Returns RefMut<T>
refcell_value.borrow_mut();
```

Cells are inherently not safe. Rust will not allow multiple threads to access
them at one once. 

There are thread-safe flavours such as Mutex, Atomics and Global Variables.

## UnsafeCell

`UnsafeCell<T>` is the core primitive building block for interior mutability in Rust.

All types that provide interior mutability are built on top of `UnsafeCell`.

Under the hood, it's `get()` method gives a raw pointer to the value it wraps. This can only be meaningfully used in unsafe blocks. The implementation (like `Mutex` or `RefCell`) that uses `UnsafeCell` usually provides a way to handle undefined behaviour.
