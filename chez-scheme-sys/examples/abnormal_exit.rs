use std::{
    ffi::{CString, c_void},
    process::exit,
};

use chez_scheme_sys::scheme::{Sregister_boot_file_bytes, Sscheme_deinit, Sscheme_init};

#[unsafe(no_mangle)]
unsafe extern "C" fn failed_setup() {
    println!("Abnormal exit!");
    exit(1);
}

fn main() {
    unsafe {
        Sscheme_init(Some(failed_setup));
        // Try to feed it a bogus boot file
        register_boot_file("fake", &[1, 1, 1, 1, 1, 1, 1, 1]);

        Sscheme_deinit();
    }
}

unsafe fn register_boot_file(name: &str, bytes: &[u8]) {
    let cstr = CString::new(name).unwrap();
    unsafe {
        Sregister_boot_file_bytes(
            cstr.as_ptr(),
            bytes.as_ptr() as *mut c_void,
            bytes.len() as isize,
        );
    }
}
