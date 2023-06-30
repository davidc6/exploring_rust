mod wrapper;

use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Clone, Debug)]
// #[repr(C)]
pub struct Person {
    first_name: String,
    last_name: String,
}

#[repr(C)]
pub struct CPerson {
    first_name: *const c_char,
    last_name: *const c_char,
}

pub enum Error {
    Failed,
}

impl Person {
    pub fn new(first_name: &str, last_name: &str) -> Result<Person, Error> {
        Ok(Person {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
        })
    }

    pub fn new_c(first_name: *const c_char, last_name: *const c_char) -> Result<CPerson, Error> {
        Ok(CPerson {
            first_name,
            last_name: CString::new("Aaha").unwrap().into_raw(),
        })
    }

    pub fn cap_first_name(&mut self) {
        let mut chars = self.first_name.chars();

        let caped = match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        };

        self.first_name = caped;
    }

    pub fn update_last_name(&'static mut self, last_name: String) {
        let initial = last_name.chars().nth(0).unwrap();
        self.last_name = format!("({:?}) {:?}", initial, last_name);
    }
}
