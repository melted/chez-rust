use std::{env::{self, args}, ffi::{CStr, CString, c_void}};

use chez_sys::{self, Sfixnum_value, Sfixnump, boot::{petite_boot, scheme_boot}, scheme::{iptr, Sbuild_heap, Scall1, Scall3, Sinteger, Sregister_boot_file_bytes, Sscheme_deinit, Sscheme_init, Sstring, Sstring_to_symbol, Stop_level_value}};

fn main() {
    unsafe {
        Sscheme_init(None);
        register_boot_file("petite", petite_boot());
        register_boot_file("scheme", scheme_boot());
        let me = args().nth(0).unwrap_or_default();
        let cme = CString::new(me).unwrap();
        Sbuild_heap(cme.as_ptr(), None);
        let hello = Sstring(c"hello".as_ptr());
        let display_sym = Sstring_to_symbol(c"display".as_ptr());
        let display_fun = Stop_level_value(display_sym);
        Scall1(display_fun, hello);
        let add = Stop_level_value(Sstring_to_symbol(c"+".as_ptr()));
        let res = Scall3(add, Sinteger(1), Sinteger(2), Sinteger(3));
        println!("{} {}", Sfixnump!(res), Sfixnum_value!(res));
        Sscheme_deinit();
    }
}

unsafe fn register_boot_file(name: &str, bytes:&[u8]) {
    let cstr = CString::new(name).unwrap();
    unsafe {
        Sregister_boot_file_bytes(cstr.as_ptr(), bytes.as_ptr() as *mut c_void, bytes.len() as isize);
    }
}