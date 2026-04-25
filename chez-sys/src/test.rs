#![cfg(test)]

use crate::{ Scar, Scdr, Schar, Schar_value, Scharp, Svector_ref, Svectorp, boot, helpers::{scheme_setup, scheme_teardown}, internal::{alloc_bignum, bignum_bigits, bignum_sign, bignum_size}, is_other, scheme::{Scons, Sfalse, Sinteger, Sinteger_value, Smake_vector, Sscheme_deinit, Sscheme_init, Svector_set, ptr, string_char, uptr}, test_heap_value
};

#[test]
fn test_embedded_boot_file() {
    let petite = boot::petite_boot();
    let scheme = boot::scheme_boot();
    assert!(petite.len() > 1000000);
    assert!(scheme.len() > 1000000)
}

#[test]
fn test_init() {
    unsafe {
        Sscheme_init(None);
    }
}

#[test]
fn test_char() {
    let m = Schar!('m');
    assert!(Scharp!(m));
    let c = unsafe { Schar_value!(m) };
    assert_eq!(c, 'm')
}

#[test]
fn test_complex() {
    let m = Schar!('m');
    unsafe { assert!(!Svectorp!(m)) };
}

#[test]
fn test_pair() {
    scheme_setup("test");
    unsafe {
        let p = Scons(Sinteger(1), Sinteger(2));
        let x = Scar!(p);
        assert_eq!(Sinteger_value(x), 1);
        let y = Scdr!(p);
        assert_eq!(Sinteger_value(y), 2);
    }
    scheme_teardown();
}

#[test]
fn test_vector() {
    scheme_setup("test");
    unsafe {
        let v = Smake_vector(5, Sinteger(8));
        let is_vector = Svectorp!(v);
        assert!(is_vector);
        Svector_set(v, 2, Sfalse);
        let n = Svector_ref!(v, 2);
        assert_eq!(n, Sfalse);
    }
    scheme_teardown();
}


#[test]
fn test_bignum_alloc() {
    scheme_setup("test");
    let bn = alloc_bignum(false, 8);
    assert_eq!(bignum_sign(bn), false);
    assert_eq!(bignum_size(bn), 8);
    dbg!(bignum_bigits(bn), bn);
    scheme_teardown();
}