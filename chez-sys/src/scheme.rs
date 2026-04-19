//! This module started as bindgen output from Chez Scheme's scheme.h.
//! Since then it has been made cross-platform, arguments have been named,
//! documentation added and the macros from the C header has been hand-translated.
//!
//! It's expected that any further changes in the API will be added to this file
//! manually.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;

pub type Sint32_t = i32;
pub type Suint32_t = u32;
pub type Sint64_t = i64;
pub type Suint64_t = u64;
pub type ptr = *mut c_void;
pub type iptr = isize;
pub type uptr = usize;
pub type xptr = ptr;
pub type string_char = u32;
pub type octet = u8;

/// Type predicate for fixnum
#[macro_export]
macro_rules! Sfixnump {
    ($e:expr) => {
        $e as usize & 0x7 == 0x0
    };
}

/// Type predicate for char
#[macro_export]
macro_rules! Scharp {
    ($e:expr) => {
        $e as usize & 0xFF == 0x16
    };
}

/// Type predicate for null
#[macro_export]
macro_rules! Snullp {
    ($e:expr) => {
        $e as uptr == 0x26
    };
}

/// Type predicate for eof object
#[macro_export]
macro_rules! Seof_objectp {
    ($e:expr) => {
        $e as uptr == 0x36
    };
}

/// Type predicate for bwp object
#[macro_export]
macro_rules! Sbwp_objectp {
    ($e:expr) => {
        $e as uptr == 0x4E
    };
}

/// Type predicate for boolean
#[macro_export]
macro_rules! Sbooleanp {
    ($e:expr) => {
        $e as uptr & 0xf7 == 0x6
    };
}

/// Type predicate for pair
#[macro_export]
macro_rules! Spairp {
    ($e:expr) => {
        $e as uptr & 0x7 == 0x1
    };
}

/// Type predicate for symbol
#[macro_export]
macro_rules! Ssymbolp {
    ($e:expr) => {
        $e as usize & 0x7 == 0x3
    };
}

/// Type predicate for procedure
#[macro_export]
macro_rules! Sprocedurep {
    ($e:expr) => {
        $e as uptr & 0x7 == 0x5
    };
}

/// Type predicate for flonum
#[macro_export]
macro_rules! Sflonump {
    ($e:expr) => {
        $e as uptr & 0x7 == 0x2
    };
}

#[macro_export]
macro_rules! is_other {
    ($e:expr) => {
         $e as uptr & 0x7 == 0x7
    };
}

#[macro_export]
macro_rules! test_heap_value {
    ($e:expr, $mask:literal, $i:literal) => {
        *(($e as uptr + 1) as *mut u64) & $mask == $i
    };
}

/// Type predicate for vector
#[macro_export]
macro_rules! Svectorp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0x07, 0x0)
    };
}

/// Type predicate for fxvector
#[macro_export]
macro_rules! Sfxvectorp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0xf, 0x3)
    };
}

/// Type predicate for flvector
#[macro_export]
macro_rules! Sflvectorp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0xf, 0xb)
    };
}

/// Type predicate for bytevector
#[macro_export]
macro_rules! Sbytevectorp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0x3, 0x1)
    };
}

/// Type predicate for string
#[macro_export]
macro_rules! Sstringp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0x7, 0x2)
    };
}

/// Type predicate for stencil vector
#[macro_export]
macro_rules! Sstencil_vectorp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0x3f, 0xe)
    };
}

/// Type predicate for system stencil vector
#[macro_export]
macro_rules! Ssystem_stencil_vectorp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0x3f, 0x2e)
    };
}

/// Type predicate for any stencil vector
#[macro_export]
macro_rules! Sany_stencil_vectorp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0x1f, 0xe)
    };
}

/// Type predicate for bignum
#[macro_export]
macro_rules! Sbignump {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0x1f, 0x6)
    };
}

/// Type predicate for box
#[macro_export]
macro_rules! Sboxp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0xff, 0x1e)
    };
}

/// Type predicate for inexact num
#[macro_export]
macro_rules! Sinexactnump {
    ($e:expr) => {
        is_other!($e) && *(($e as uptr + 1) as ptr) == 0x36
    };
}

// Type predicate for exact num
#[macro_export]
macro_rules! Sexactnump {
    ($e:expr) => {
        is_other!($e) && *(($e as uptr + 1) as ptr) == 0x56
    };
}

// Type predicate for ratnum
#[macro_export]
macro_rules! Sratnump {
    ($e:expr) => { 
        is_other!($e) && *(($e as uptr + 1) as ptr) == 0x16
    };
}

// Type predicate for input port
#[macro_export]
macro_rules! Sinputportp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0x1ff, 0x1de)
    };
}

// Type predicate for output port
#[macro_export]
macro_rules! Soutputportp {
    ($e:expr) => {
        is_other!($e) && test_heap_value!($e, 0x2ff, 0x2de)
    };
}

/// Type predicate for record
#[macro_export]
macro_rules! Srecordp {
    ($e:expr) => { 
        is_other!($e) && test_heap_value!($e, 0x7, 0x7)
    };
}

/// Get fixnum value from ptr
#[macro_export]
macro_rules! Sfixnum_value {
    ($e:expr) => {
        ($e as iptr) / 8
    };
}

/// Get char value from ptr
#[macro_export]
macro_rules! Schar_value {
    ($e:expr) => {
        char::from_u32_unchecked((($e as uptr) >> 8) as u32)
    };
}

/// Get boolean value from ptr
#[macro_export]
macro_rules! Sboolean_value {
    ($e:expr) => {
        $e != Sfalse
    };
}

/// Get car of pair
#[macro_export]
macro_rules! Scar {
    ($e:expr) => {
        *((($e as uptr) + 7) as *mut ptr)
    };
}

/// Get cdr of pair
#[macro_export]
macro_rules! Scdr {
    ($e:expr) => {
        *((($e as uptr) + 15) as *mut ptr)
    };
}

/// Get a flonum value from ptr
#[macro_export]
macro_rules! Sflonum_value {
    ($e:expr) => {
        *((($e as uptr) + 6) as *const f64)
    };
}

/// Get the length of a vector
#[macro_export]
macro_rules! Svector_length {
    ($e:expr) => {
        *((($e as uptr) + 1) as *const iptr) >> 4
    };
}

/// Get an element of a vector
#[macro_export]
macro_rules! Svector_ref {
    ($e:expr, $i:expr) => {
        *(((($e as uptr) + 9) as *mut ptr).add($i))
    };
}

/// Get the length of an fxvector
#[macro_export]
macro_rules! Sfxvector_length {
    ($e:expr) => {
        *((($e as uptr) + 1) as *const iptr) >> 4
    };
}

/// Get an element of an fxvector
#[macro_export]
macro_rules! Sfxvector_ref {
    ($e:expr, $i:expr) => {
        *(((($e as uptr) + 9) as *mut iptr).add($i))
    };
}

/// Get the length of an flvector
#[macro_export]
macro_rules! Sflvector_length {
    ($e:expr) => {
        *((($e as uptr) + 1) as *const iptr) >> 4
    };
}

/// Get an element of an flvector
#[macro_export]
macro_rules! Sflvector_ref {
    ($e:expr, $i:expr) => {
        *(((($e as uptr) + 9) as *const f64).add($i))
    };
}

/// Get the length of a bytevector
#[macro_export]
macro_rules! Sbytevector_length {
    ($e:expr) => {
        *((($e as uptr) + 1) as *const iptr) >> 3
    };
}

/// Get an element of a bytevector
#[macro_export]
macro_rules! Sbytevector_u8_ref {
    ($e:expr, $i:expr) => {
        *(((($e as uptr) + 9) as *const u8).add($i))
    };
}

/// Get the data of a bytevector
/// Warning: Sbytevector_data(x) returns a pointer into x.
#[macro_export]
macro_rules! Sbytevector_data {
    ($e:expr) => {
        ((($e as uptr) + 9) as *const u8)
    };
}

/// Get the length of a string
#[macro_export]
macro_rules! Sstring_length {
    ($e:expr) => {
        *((($e as uptr) + 1) as *const iptr) >> 4
    };
}

/// Get an element of a vector
#[macro_export]
macro_rules! Sstring_ref {
    ($e:expr, $i:expr) => {
        Schar_value(*(((($e as uptr) + 9) as *const string_char).add($i)))
    };
}

/// Unbox a box
#[macro_export]
macro_rules! Sunbox {
    ($e:expr) => {
        *((($e as uptr) + 9) as ptr)
    };
}

/// Get the length of a stencil vector
#[macro_export]
macro_rules! Sstencilvector_length {
    ($e:expr) => {
        Spopcount(*((($e as uptr) + 1) as *const iptr) >> 6)
    };
}

/// Get an element of a stencil vector
#[macro_export]
macro_rules! Sstencilvector_ref {
    ($e:expr, $i:expr) => {
        *(((($e as uptr) + 9) as ptr).add($i))
    };
}

/// Get an unsigned integer from ptr
#[macro_export]
macro_rules! Sunsigned_value {
    ($e:expr) => {
        Sinteger_value($e) as uptr
    };
}

/// Get an unsigned integer from ptr
#[macro_export]
macro_rules! Sunsigned32_value {
    ($e:expr) => {
        Sinteger32_value($e) as u32
    };
}

/// Get an unsigned integer from ptr
#[macro_export]
macro_rules! Sunsigned64_value {
    ($e:expr) => {
        Sinteger64_value($e) as u64
    };
}

/// Set a char in a string.
/// TODO: check how it's working with native Rust chars
#[macro_export]
macro_rules! Sstring_set {
    ($e:expr, $i:expr, $c:expr) => {
        *(((($e as uptr) + 9) as *mut string_char).add($i)) = Schar(c);
    };
}

/// Set a fixnum in an fxvector.
/// TODO: check how it's working with native Rust chars
#[macro_export]
macro_rules! Sfxvector_set {
    ($e:expr, $i:expr, $n:expr) => {
        Sfxvector_ref($e, $i) = n;
    };
}

/// Set a float in an flvector.
/// TODO: check how it's working with native Rust types
#[macro_export]
macro_rules! Sflvector_set {
    ($e:expr, $i:expr, $n:expr) => {
        Sflvector_ref($e, $i) = n;
    };
}

/// Set a byte in a bytevector.
/// TODO: check how it's working with native Rust chars
#[macro_export]
macro_rules! Sbytevector_u8_set {
    ($e:expr, $i:expr, $n:expr) => {
        Sbytevector_u8_ref($e, $i) = n;
    };
}

#[macro_export]
macro_rules! Srecord_uniform_ref {
    ($e:expr, $i:expr) => {
        *(((($e as uptr) + 9) as ptr).add($i))
    };
}

#[macro_export]
macro_rules! Sforeign_callable_entry_point {
    ($e:expr) => {
        ((($e as uptr) + 65) as *mut fn())
    };
}

#[macro_export]
macro_rules! Sforeign_callable_code_object {
    ($e:expr) => {
        ((($e as uptr) - 65) as ptr)
    };
}

#[macro_export]
macro_rules! Sfixnum {
    ($e:expr) => {
        (($e as uptr) * 8) as ptr
    };
}

#[macro_export]
macro_rules! Schar {
    ($e:expr) => {
        ((($e as u32) << 8) | 0x16) as ptr
    }
}

pub const Snil: ptr = 0x26 as ptr;
pub const Strue: ptr = 0xe as ptr;
pub const Sfalse: ptr = 0x6 as ptr;

#[macro_export]
macro_rules! Sboolean {
    ($e:expr) => {
        if $e { Strue } else { Sfalse }
    };
}

pub const Sbwp_object: ptr = 0x4e as ptr;
pub const Seof_object: ptr = 0x36 as ptr;
pub const Svoid: ptr = 0x2e as ptr;

unsafe extern "C" {

    pub fn Sinteger32_value(integer: ptr) -> Sint32_t;
    pub fn Sinteger_value(integer: ptr) -> iptr;
    pub fn Sinteger64_value(integer: ptr) -> Sint64_t;
    pub fn Stry_integer_value(
        integer: ptr,
        result: *mut iptr,
        reason: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Stry_integer32_value(
        integer: ptr,
        result: *mut Sint32_t,
        reason: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Stry_integer64_value(
        integer: ptr,
        result: *mut Sint64_t,
        reason: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Stry_unsigned_value(
        integer: ptr,
        result: *mut uptr,
        reason: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Stry_unsigned32_value(
        integer: ptr,
        result: *mut Suint32_t,
        reason: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Stry_unsigned64_value(
        integer: ptr,
        result: *mut Suint64_t,
        reason: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Sset_box(the_box: ptr, obj: ptr);

    pub fn Sset_car(pair: ptr, obj: ptr);

    pub fn Sset_cdr(pair: ptr, obj: ptr);

    pub fn Svector_set(vec: ptr, i: iptr, obj: ptr);

    pub fn Scons(arg1: ptr, arg2: ptr) -> ptr;

    pub fn Sstring_to_symbol(s: *const ::std::os::raw::c_char) -> ptr;

    pub fn Ssymbol_to_string(arg1: ptr) -> ptr;

    pub fn Sflonum(x: f64) -> ptr;

    pub fn Smake_vector(n: iptr, obj: ptr) -> ptr;

    pub fn Smake_fxvector(n: iptr, fixnum: ptr) -> ptr;

    pub fn Smake_flvector(n: iptr, flonum: f64) -> ptr;

    pub fn Smake_bytevector(n: iptr, fill: ::std::os::raw::c_int) -> ptr;

    pub fn Smake_string(n: iptr, c: ::std::os::raw::c_int) -> ptr;

    pub fn Smake_uninitialized_string(n: iptr) -> ptr;

    pub fn Sstring(s: *const ::std::os::raw::c_char) -> ptr;

    pub fn Sstring_of_length(s: *const ::std::os::raw::c_char, n: iptr) -> ptr;

    pub fn Sstring_utf8(s: *const ::std::os::raw::c_char, n: iptr) -> ptr;

    pub fn Sbox(obj: ptr) -> ptr;

    pub fn Sinteger(n: iptr) -> ptr;

    pub fn Sunsigned(n: uptr) -> ptr;

    pub fn Sinteger32(n: Sint32_t) -> ptr;

    pub fn Sunsigned32(n: Suint32_t) -> ptr;

    pub fn Sinteger64(n: Sint64_t) -> ptr;

    pub fn Sunsigned64(n: Suint64_t) -> ptr;

    pub fn Srecord_type(rec: ptr) -> ptr;

    pub fn Srecord_type_parent(rtd: ptr) -> ptr;

    pub fn Srecord_type_uniformp(rtd: ptr) -> ::std::os::raw::c_int;

    pub fn Srecord_type_size(rtd: ptr) -> uptr;

    pub fn Stop_level_value(sym: ptr) -> ptr;

    pub fn Sset_top_level_value(sym: ptr, obj: ptr);

    /// In practice, the best way to ensure that C code does not retain pointers to Scheme
    /// objects is to immediately convert the Scheme objects into C equivalents, if
    /// possible. In certain cases, it is not possible to do so, yet retention of the Scheme
    /// object is essential to the design of the C portions of the program. In these cases,
    /// the object may be locked via the library routine Slock_object (or from Scheme, the
    /// equivalent procedure lock-object).

    /// Locking an object prevents the storage manager from reclaiming or relocating the
    /// object. Locking should be used sparingly, as it introduces memory fragmentation and
    /// increases storage management overhead. Locking can also lead to accidental retention
    /// of storage if objects are not unlocked. Locking objects that have been made static
    /// via heap compaction (see Scompact_heap above) is unnecessary but harmless.
    ///
    /// An object may be locked more than once by successive calls to Slock_object or
    /// lock-object, in which case it must be unlocked by an equal number of calls to
    /// Sunlock_object or unlock-object before it is truly unlocked.
    pub fn Slock_object(obj: ptr);

    /// Objects may be unlocked via Sunlock_object (unlock-object).
    pub fn Sunlock_object(obj: ptr);

    /// The function Slocked_objectp can be used to determine if an object is locked.
    pub fn Slocked_objectp(obj: ptr) -> ::std::os::raw::c_int;

    /// Foreign entry points may be made visible to Scheme via Sforeign_symbol or
    /// Sregister_symbol.
    ///
    /// External entry points in object files or shared objects loaded as a result of a call
    /// to load-shared-object are automatically made visible by the system. Once a foreign
    /// entry point is made visible, it may be named in a foreign-procedure expression to
    /// create a Scheme-callable version of the entry point. Sforeign_symbol and
    /// Sregister_symbol allow programs to register nonexternal entry points, entry points
    /// in code linked statically with Chez Scheme, and entry points into code loaded
    /// directly from C, i.e., without load-shared-object. Sforeign_symbol and
    /// Sregister_symbol differ only in that Sforeign_symbol raises an exception when an
    /// attempt is made to register an existing name, whereas Sregister_symbol permits
    /// existing names to be redefined.
    pub fn Sforeign_symbol(name: *const ::std::os::raw::c_char, addr: *mut ::std::os::raw::c_void);

    /// Like Sforeign_symbol, but allows symbols to be redefined
    pub fn Sregister_symbol(name: *const ::std::os::raw::c_char, addr: *mut ::std::os::raw::c_void);

    /// Call a Scheme function with no arguments
    pub fn Scall0(procedure: ptr) -> ptr;

    /// Call a Scheme function with one arguments
    pub fn Scall1(procedure: ptr, obj1: ptr) -> ptr;

    /// Call a Scheme function with two arguments
    pub fn Scall2(procedure: ptr, obj1: ptr, obj2: ptr) -> ptr;

    /// Call a Scheme function with three arguments
    pub fn Scall3(procedure: ptr, obj1: ptr, obj2: ptr, obj3: ptr) -> ptr;

    /// A C procedure first calls Sinitframe with one argument, the number of arguments to
    /// be passed to Scheme. It then calls Sput_arg once for each argument (in any order),
    /// passing Sput_arg the argument number (starting with 1) and the argument. Finally, it
    /// calls Scall to perform the call, passing it the Scheme procedure and the number of
    /// arguments (the same number as in the call to Sinitframe). Programmers should ensure
    /// a Scheme call initiated via Sinitframe is completed via Scall before any other calls
    /// to Scheme are made and before a return to Scheme is attempted. If for any reason the
    /// call is not completed after Sinitframe has been called, it may not be possible to
    /// return to Scheme.
    pub fn Sinitframe(n: iptr);

    pub fn Sput_arg(i: iptr, obj: ptr);

    pub fn Scall(procedure: ptr, n: iptr) -> ptr;

    /// Skernel_version returns a string representing the Scheme version. It should be
    /// compared against the value of the VERSION preprocessor macro before any of the
    /// initialization functions listed above are used to verify that the correct "scheme.h"
    /// header file has been used.
    pub fn Skernel_version() -> *const ::std::os::raw::c_char;

    /// Sretain_static_relocation causes relocation information to be retained for static
    /// generation code objects created by heap compaction for the benefit of compute-size
    /// and related procedures.
    pub fn Sretain_static_relocation();

    /// Sset_verbose sets verbose mode on for nonzero values of v and off when v is zero. In
    /// verbose mode, the system displays a trace of the search process for subsequently
    /// registered boot files.
    pub fn Sset_verbose(v: ::std::os::raw::c_int);

    /// Sscheme_init causes the Scheme system to initialize its static memory in preparation
    /// for boot file registration. The _abnormal_exit_ parameter should be a
    /// (possibly null) pointer to a C function of no arguments that takes appropriate
    /// action if the initialization or subsequent heap-building process fails.
    /// If null, the default action is to call exit(1).
    pub fn Sscheme_init(abnormal_exit: ::std::option::Option<unsafe extern "C" fn()>);

    /// Sregister_boot_file searches for a boot file and registers it for loading. If the
    /// given boot file path is absolute or starts with a . or .. path element, then the
    /// path is used relative to the current directory; otherwise, the boot file is found
    /// through a search as described in Section 2.9, and "scheme" is assumed as the
    /// executable name for resolving a "%x" escape when the executable path is not
    /// otherwise available from the operating system. For the first boot file registered
    /// only, the system also searches for the boot files upon which the named file depends,
    /// either directly or indirectly.
    pub fn Sregister_boot_file(name: *const ::std::os::raw::c_char);

    /// register_boot_executable_relative_file is similar, but accepts a path to the
    /// executable instead of assuming "scheme", so it should be preferred when using a boot
    /// file path that may be resolved relative to the executable.
    pub fn Sregister_boot_executable_relative_file(
        execpath: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    );

    /// Sregister_boot_relative_file always treats a relative path as relative to the
    /// current directory, so the executable path is not needed. In all of those cases, the
    /// boot file is opened but not loaded until the heap is built via Sbuild_heap.
    pub fn Sregister_boot_relative_file(name: *const ::std::os::raw::c_char);

    /// Sregister_boot_file_fd provides a specific boot file as a file descriptor, the given
    /// file name is used only for error reporting, and the file descriptor is not read
    /// until until the heap is built via Sbuild_heap.
    pub fn Sregister_boot_file_fd(name: *const ::std::os::raw::c_char, fd: ::std::os::raw::c_int);

    /// The Sregister_boot_file_fd_region function is an alternative to
    /// Sregister_boot_file_fd that allows the same file descriptor to be used for multiple
    /// boot files using different offsets into the file. The len argument is used as a
    /// hint, but it can be 0 to mean that the size is unknown, or it can be larger than the
    /// actual boot content; it must not be non-0 and smaller than the boot content, and the
    /// boot content must be self-terminating independent of len. No search is performed for
    /// dependencies. If the same file descriptor is used for multiple boot files, then
    /// close_after should be non-zero only for the last one. The boot file content is read
    /// only when Sbuild_heap is called.
    pub fn Sregister_boot_file_fd_region(
        name: *const ::std::os::raw::c_char,
        fd: ::std::os::raw::c_int,
        offset: iptr,
        len: iptr,
        close_after: ::std::os::raw::c_int,
    );

    /// The Sregister_boot_file_bytes function is another alternative to the
    /// Sregister_boot_file functions that registers boot-file content that is already
    /// loaded into memory, instead of reading from a file. The registered bytes must remain
    /// available until Sbuild_heap reads them.
    pub fn Sregister_boot_file_bytes(
        name: *const ::std::os::raw::c_char,
        content: *mut ::std::os::raw::c_void,
        len: iptr,
    );

    // Not used
    // pub fn Sregister_heap_file(arg1: *const ::std::os::raw::c_char);

    pub fn Scompact_heap();

    // Not used
    //pub fn Ssave_heap(arg1: *const ::std::os::raw::c_char, arg2: ::std::os::raw::c_int);

    pub fn Sbuild_heap(
        exec: *const ::std::os::raw::c_char,
        custom_init: ::std::option::Option<unsafe extern "C" fn()>,
    );

    pub fn Senable_expeditor(history_file: *const ::std::os::raw::c_char);

    pub fn Sscheme_start(
        argc: ::std::os::raw::c_int,
        argv: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Sscheme_script(
        scriptfile: *const ::std::os::raw::c_char,
        argc: ::std::os::raw::c_int,
        argv: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Sscheme_program(
        programfile: *const ::std::os::raw::c_char,
        argc: ::std::os::raw::c_int,
        argv: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Sscheme_deinit();

    pub fn Sscheme_register_signal_registerer(
        f: ::std::option::Option<unsafe extern "C" fn(sig: ::std::os::raw::c_int)>,
    );

    pub fn Sactivate_thread() -> ::std::os::raw::c_int;

    pub fn Sdeactivate_thread();

    pub fn Sdestroy_thread() -> ::std::os::raw::c_int;

    /// Sgetenv returns the UTF-8-encoded value of UTF-8-encoded environment variable name
    /// if found and NULL otherwise. Call free on the returned value when it is no longer
    /// needed.
    #[cfg(target_os = "windows")]
    pub fn Sgetenv(name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;

    /// Sutf8_to_wide and Swide_to_utf8 convert between UTF-8-encoded and UTF-16LE-encoded
    /// null-terminated strings. Call free on the returned value when it is no longer needed.
    #[cfg(target_os = "windows")]
    pub fn Sutf8_to_wide(s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_ushort;

    #[cfg(target_os = "windows")]
    pub fn Swide_to_utf8(s: *const ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_char;
}
