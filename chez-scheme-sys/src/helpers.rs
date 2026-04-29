use std::ffi::{CString, c_void};
use crate::{boot::{petite_boot, scheme_boot}, 
            scheme::{Sbuild_heap, Sregister_boot_file_bytes, Sscheme_deinit, Sscheme_init}};

pub fn scheme_setup(arg0 : &str) {
    unsafe {
        Sscheme_init(None);
        register_boot_file("petite", petite_boot());
        register_boot_file("scheme", scheme_boot());
        let cme = CString::new(arg0).unwrap();
        Sbuild_heap(cme.as_ptr(), None);
    }
}

pub fn scheme_teardown() {
    unsafe {
        Sscheme_deinit();
    }
}

pub fn register_boot_file(name: &str, bytes: &[u8]) {
    let cstr = CString::new(name).unwrap();
    unsafe {
        Sregister_boot_file_bytes(
            cstr.as_ptr(),
            bytes.as_ptr() as *mut c_void,
            bytes.len() as isize,
        );
    }
}