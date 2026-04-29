//! Internal Chez Scheme APIs.
//! 
//! The C API as documented in the Chez Scheme Users Guide lack functions
//! for allocating and inspecting bignums, and there is no way to efficiently
//! do it via Scheme functions. So this module uses nonpublic functions for
//! bignum allocation and inspection. This can be used to efficiently convert
//! Scheme bignums to and from Rust bignums. That isn't provided here, since
//! I want to keep this library dependency-free. 
#![warn(nonstandard_style)]

use std::ffi::c_void;

const CENTRY_thread_context:isize = 0;

unsafe extern "C" {

    pub  fn S_lookup_c_entry(i : isize) -> *mut c_void;
    pub fn S_bignum(tc: *mut c_void, n:isize, sign:i32) -> *mut c_void;
}

pub fn get_thread_context() -> *mut c_void {
    unsafe { S_lookup_c_entry(CENTRY_thread_context) }
}

pub fn alloc_bignum(sign:bool, n:isize) -> *mut c_void {
    let tc = get_thread_context();
    unsafe {
        S_bignum(tc, n, if sign { 1 } else { 0 })
    }
}

pub fn bignum_sign(bn : *mut c_void) -> bool {
    unsafe {
        let header = *(((bn as usize) + 1) as *mut usize);
        header & 0x20 == 0x20
    }
}

pub fn bignum_size(bn : *mut c_void) -> usize {
    unsafe {
        let header = *(((bn as usize) + 1) as *mut usize);
        header >> 6
    }
}

pub fn bignum_bigits(bn : *mut c_void) -> *mut u32 {
    let bigits = ((bn as usize) + 9) as *mut u32;
    bigits
}