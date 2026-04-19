#![cfg(test)]

use crate::{
    Schar_value, Scharp, boot, scheme::{Schar, Sscheme_deinit, Sscheme_init, string_char, uptr}
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
    let m = Schar('m');
    assert!(Scharp!(m));
    let c = Schar_value!(m);
    dbg!(c);
}