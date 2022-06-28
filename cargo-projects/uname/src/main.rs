// libc (c common library)
// libc crate is raw ffi bindings that enable us to easily interop with C code on Rust supported platforms
// it includes type definitions, constants, function headers 
// uname - short for Unix Name (a program that prints details about the machine)
// utsname - a structure that returns system information
use libc::{uname, utsname};
// this wrapper type allows us to construct uninitialized instances of a type
// https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html
use std::mem::MaybeUninit;
// this clone-on-write (Cow) smart pointer enables us to work with borrowed data
use std::borrow::Cow;
// core io functinality
use std::io;
// representation of borrowed C string
// borrowed reference to a nul-terminated array of bytes
use std::ffi::CStr;

trait Uname {
    fn machine(&self) -> Cow<str>;
    fn system(&self) -> Cow<str>;
}

#[derive(Copy, Clone)]
struct PlatInfo {
    values: utsname
}

impl PlatInfo {
    pub fn new() -> io::Result<Self> {
        unsafe {
            // explicity uninitialised reference
            // since the compiler knows that data inside may be invalid
            // and it is not undefined behaviour
            // MaybeUninit<T> enables unsafe code to deal with uninitialized data,
            // which let's the compiler know that the data here might not be initialized
            let mut x = MaybeUninit::<utsname>::uninit();
            // a mutable pointer gets passed and in order for it to put result into
            if uname(x.as_mut_ptr()) != -1 {
                // once data is initialised with real values we call assume_init()
                Ok(Self { values: x.assume_init() })
            } else {
                // represents last OS error that occurred
                Err(io::Error::last_os_error())
            }
        }
    }
}

// macro_rules! syntax allows us to create macros
// arguments are usually prefixed using $
macro_rules! cstr2cow {
    // $v is an argument which is an expression
    ($v:expr) => {
        // CStr - this type represents a borrowed ref to nul-terminated array of bytes
        // since we are calling functions over FFI (foreign function interface) we need to declare this inside the unsafe block
        // CStr::from_ptr wraps C string with a safe C sting wrapper
        // which enables for inspection and interop of non-owned C strings
        // .as_ref() - convert to shared reference
        // .as_ptr() - returns a raw pointer to slice's buffer
        // .to_string_lossy() - comverts CStr into Cow<str>, which is clone-on-write smart pointer
        unsafe { CStr::from_ptr($v.as_ref().as_ptr()).to_string_lossy() }
    };
}

impl Uname for PlatInfo {
    fn machine(&self) -> Cow<str> {
        return cstr2cow!(self.values.machine);
    }

    fn system(&self) -> Cow<str> {
        return cstr2cow!(self.values.sysname);
    }
}

fn stringify(x: std::io::Error) -> String { format!("error code: {x}") }

fn main() {
    let x = PlatInfo::new().map_err(stringify).unwrap();
    println!("System: {:?} \nArchitecture: {:?}", x.system(), x.machine());
}
