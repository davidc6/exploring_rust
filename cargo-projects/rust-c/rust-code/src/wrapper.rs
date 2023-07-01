use crate::{CPerson, Person};
use std::ffi::CString;
use std::ffi::{c_char, CStr};
use std::ops::{Deref, DerefMut};

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
    person: *mut *mut Person, // mutable reference to a mutable reference of type Person which allows to indirectly access and modify value pointed by inner pointer
) -> PersonStatus {
    // "from_ptr" allows us to wrap a raw C string pointer then "to_str" (yield) a string slice
    let f_name_cstr = CStr::from_ptr(f_name).to_str().unwrap();
    let l_name_cstr = CStr::from_ptr(l_name).to_str().unwrap();

    // create Person object
    match Person::new(f_name_cstr, l_name_cstr) {
        Ok(r) => {
            // access a pointer and modify the value
            // by converting Box(ed) value into raw pointer
            // the memory management then become the responsibility of the caller
            *person = Box::into_raw(Box::new(r));
            PersonStatus::Success
        }
        Err(e) => PersonStatus::Fail,
    }
}

#[no_mangle]
/// \brief Initializes Person
///
/// \param[in] f_name first name
/// \param[in] l_name last name
/// \param[out] person Pointer to Person component
///
/// \return PersonStatus error code (PERSON_STATUS_SUCCESS) on no error
pub unsafe extern "C" fn Person_c_new(
    f_name: *const c_char,
    l_name: *const c_char,
    person: *mut *mut CPerson, // mutable reference to a mutable reference of type Person which allows to indirectly access and modify value pointed by inner pointer
) -> PersonStatus {
    // "from_ptr" allows us to wrap a raw C string pointer then "to_str" (yield) a string slice
    // let f_name_cstr = CStr::from_ptr(f_name)
    let l_name_cstr = CStr::from_ptr(l_name).to_str().unwrap();

    // create Person object
    match Person::new_c(f_name, l_name) {
        Ok(r) => {
            // access a pointer and modify the value
            // by converting Box(ed) value into raw pointer
            // the memory management then become the responsibility of the caller
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

    let prefix = CString::new("Original Person f name".to_owned()).unwrap();
    let f_name = CString::new(person_box2.first_name.to_owned()).unwrap();

    update_cb(prefix.as_ptr(), f_name.as_ptr());

    let l = Box::leak(person_box);
    // let l_c = l.clone();

    // leaking the box allows to get a 'static reference and it is kept until the program exits
    Person::cap_first_name(l);

    // let d = l;

    let first_name = CString::new(l.first_name.clone()).unwrap();
    let prefix = CString::new("Updated Person f name".to_owned()).unwrap();

    // TODO: send prefix to the callback function
    update_cb(prefix.as_ptr(), first_name.as_ptr());
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

    let prefix = CString::new("Original Person l name".to_owned()).unwrap();

    update_cb(prefix.as_ptr(), last_name);

    let c_str = unsafe { CStr::from_ptr(last_name) };
    let s_slice = c_str.to_str().unwrap();

    let person_box = Box::from_raw(person);
    let person_box2 = person_box.clone();
    Person::update_last_name(Box::leak(person_box), s_slice.to_owned());

    let prefix = CString::new("Updated Person l name".to_owned()).unwrap();

    let l_name = CString::new(person_box2.last_name).unwrap();

    update_cb(prefix.as_ptr(), l_name.as_ptr());
}
