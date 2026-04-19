//! This module contains the boot files from Chez Scheme embedded as arrays.
//! They can be used with Sread_bootfiles_bytes to read them in from the
//! executable.
//!

pub const fn petite_boot() -> &'static [u8] {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/petite.boot"));
    bytes
}

pub const fn scheme_boot() -> &'static [u8] {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/scheme.boot"));
    bytes
}
