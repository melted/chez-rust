#![cfg(test)]

use crate::boot;

#[test]
fn test_embedded_boot_file() {
    let petite = boot::petite_boot();
    let scheme = boot::scheme_boot();
    assert!(petite.len() > 1000000);
    assert!(scheme.len() > 1000000)
}

