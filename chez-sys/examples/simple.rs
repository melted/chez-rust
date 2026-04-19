use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use chez_sys::{
    self, Sfixnump, boot,
    scheme::{Skernel_version, Sscheme_init},
};

fn main() {
    let n = boot::petite_boot().len();
    println!("{n}");
    let is_fixnum = Sfixnump!(0x677);
    println!("{is_fixnum}");
    unsafe {
        let version = Skernel_version();
        let ver_str = CStr::from_ptr(version);
        println!("{}", ver_str.to_string_lossy());
    }
}
