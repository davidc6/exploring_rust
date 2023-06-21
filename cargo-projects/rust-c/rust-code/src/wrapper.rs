use crate::Person;
use std::ffi::CString;
use std::ffi::{c_char, CStr};

#[repr(C)]
pub enum PersonStatus {
    Success,
    Fail,
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
    person: *mut *mut Person,
) -> PersonStatus {
    let f_name_cstr = CStr::from_ptr(f_name).to_str().unwrap();
    let l_name_cstr = CStr::from_ptr(l_name).to_str().unwrap();

    match Person::new(f_name_cstr, l_name_cstr) {
        Ok(r) => {
            *person = Box::into_raw(Box::new(r));
            PersonStatus::Success
        }
        Err(e) => PersonStatus::Fail,
    }
}

#[no_mangle]
pub unsafe extern "C" fn person_cap_first_name(person: *mut Person, update_cb: UpdateCb) {
    if person.is_null() {
        // TODO callback
    }

    // convert raw pointer to a Box
    let person_box = Box::from_raw(person);
    let person_box2 = person_box.clone();
    // leaking the box here allows to get a 'static reference and it is kept until the program exits
    Person::cap_first_name(Box::leak(person_box));

    let last_name = CString::new(person_box2.last_name).unwrap();

    let prefix = CString::new("First name".to_owned()).unwrap();

    // TODO: send prefix to the callback function
    update_cb(prefix.as_ptr(), last_name.as_ptr());
}

pub type UpdateCb = unsafe extern "C" fn(prefix: *const c_char, last_name: *const c_char);

#[no_mangle]
pub unsafe extern "C" fn person_update_last_name(
    person: *mut Person,
    last_name: *const c_char,
    update_cb: UpdateCb,
) {
    if person.is_null() {
        // TODO
    }

    let c_str = unsafe { CStr::from_ptr(last_name) };
    let s_slice = c_str.to_str().unwrap();

    let person_box = Box::from_raw(person);
    Person::update_last_name(Box::leak(person_box), s_slice.to_owned());

    let prefix = CString::new("Last name".to_owned()).unwrap();

    update_cb(prefix.as_ptr(), last_name);
}
