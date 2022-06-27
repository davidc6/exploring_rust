// libc (c common library)
// libc crate is raw ffi bindings that enable us to easily interop with C code on Rust supported platforms
// it includes type definitions, constants, function headers 
// uname - short for Unix Name (a program that prints details about the machine)
// utsname - a structure that returns system information
use libc::{uname, utsname};
// this wrapper type allows us to construct uninitialized instances
use std::mem::MaybeUninit;
// this clone-on-write (Cow) smart pointer enables us to work with borrowed data
use std::borrow::Cow;
// core io functinality
use std::io;
// representation of borrowed C string
use std::ffi::CStr;
 
trait Uname {
    fn machine(&self) -> Cow<str>;
}

struct PlatInfo {
    value: utsname
}

impl PlatInfo {
    pub fn new() -> io::Result<Self> {
        unsafe {
            let mut x = MaybeUninit::<utsname>::uninit();
            if uname(x.as_mut_ptr()) != -1 {
                Ok(Self { value: x.assume_init() })
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }
}

// macros in Rust are create using macro_rules! syntax
// arguments are usually prefixed using $
macro_rules! cstr2cow {
    ($v:expr) => {
        // since we are calling functions over FFi we need to declare this inside the unafe block
        // wrap C string with a safe C sting wrapper
        unsafe { CStr::from_ptr($v.as_ref().as_ptr()).to_string_lossy() }
    };
}

impl Uname for PlatInfo {
    fn machine(&self) -> Cow<str> {
        return cstr2cow!(self.value.machine);
    }
}

fn stringify(x: std::io::Error) -> String { format!("error code: {x}") }

fn main() {
    let x = 
        PlatInfo::new()
            .map_err(stringify);
    println!("{:?}", x.unwrap().machine());
}
