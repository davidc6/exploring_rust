use std::ffi::{CStr, c_char};

use crate::Person;

#[repr(C)]
pub enum PersonStatus {
    Success,
    Fail
}

#[no_mangle]
/// \brief Initializes Person
/// 
/// \param[in] f_name first name
/// \param[in] l_name last name
/// \param[out] person Pointer to Person component
/// 
/// \return PersonStatus error code (PERSON_STATUS_SUCCESS) on no error
pub unsafe extern "C" fn Person_new(
    f_name: *const c_char,
    l_name: *const c_char,
    person: *mut *mut Person
) -> PersonStatus {
    let f_name_cstr = CStr::from_ptr(f_name).to_str().unwrap();
    let l_name_cstr = CStr::from_ptr(l_name).to_str().unwrap();

    match Person::new(f_name_cstr, l_name_cstr) {
        Ok(r) => {
            *person = Box::into_raw(Box::new(r));
            PersonStatus::Success
        },
        Err(e) => PersonStatus::Fail
    }
}

#[no_mangle]
pub unsafe extern "C" fn person_cap_first_name(person: *mut Person) {
    if person.is_null() {
        // TODO callback
    }

    let person_box = Box::from_raw(person);
    Person::cap_first_name(Box::leak(person_box));
}

pub type UpdateLastNameCb = unsafe extern "C" fn(
    last_name: *const c_char
);

#[no_mangle]
pub unsafe extern "C" fn person_update_last_name(person: *mut Person, last_name: *const c_char, update_cb: UpdateLastNameCb) {
    if person.is_null() {
        // TODO
    }
    
    let c_str = unsafe { CStr::from_ptr(last_name) };
    let s_slice = c_str.to_str().unwrap();

    let person_box = Box::from_raw(person);
    Person::update_last_name(Box::leak(person_box), s_slice.to_owned());

    update_cb(last_name);
}
