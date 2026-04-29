use std::{ffi::CStr, ptr::copy_nonoverlapping};

use chez_scheme_sys::{Sboolean_value, Sbytevector_data, Sbytevector_length, Sstring_length, scheme::{Scall0, Scall1, Scall2, Scons, Smake_bytevector, Snil, Sstring_to_symbol, Sstring_utf8, Stop_level_value, iptr, ptr, uptr}};


pub fn get_top_level_object(name: &CStr) -> ptr {
    unsafe {
        let sym = symbol(name);
        Stop_level_value(sym)
    }
}

pub fn is_top_level_bound(name: &CStr) -> bool {
    let top_level_bound_proc = get_top_level_object(c"top-level-bound?");
    unsafe {
        let sym = Sstring_to_symbol(name.as_ptr());
        let res = Scall1(top_level_bound_proc, sym);
        Sboolean_value!(res)
    }
}

pub fn make_list(objs : &[ptr]) -> ptr {
    let mut list = Snil;
    for o in objs.iter().rev() {
        list = unsafe { Scons(*o, list) };
    }
    list
}

pub fn quote(obj: ptr) -> ptr {
    let quote = symbol(c"quote");
    make_list(&[quote, obj])
}

pub fn symbol(name: &CStr) -> ptr {
    unsafe { Sstring_to_symbol(name.as_ptr()) }
}


pub fn eval(obj:ptr) -> ptr {
    let eval = get_top_level_object(c"eval");
    unsafe {
        Scall1(eval, obj)
    }
}

pub fn compile_scheme_code(code:&str) -> ptr {
    unsafe {
        let compile_to_port = get_top_level_object(c"compile-port");
        let load_from_port = get_top_level_object(c"load-compiled-from-port");
        let scm_code = Sstring_utf8(code.as_ptr() as *const i8, code.len() as isize);
        let open_input_string = get_top_level_object(c"open-input-string");
        let open_bytes_input = get_top_level_object(c"open-bytevector-input-port");
        let open_output_bytes = get_top_level_object(c"open-bytevector-output-port");
        let call_with_values = get_top_level_object(c"call-with-values");
        let list = get_top_level_object(c"list");
        let cadr = get_top_level_object(c"cadr");
        let car = get_top_level_object(c"car");
        let input_port = Scall1(open_input_string, scm_code);
        let output_res = Scall2(call_with_values, open_output_bytes, list);
        let output_port = Scall1(car, output_res);
        let extract = Scall1(cadr, output_res);
        let _compile = Scall2(compile_to_port, input_port, output_port);
        let bytes = Scall0(extract);
        let bytes_input = Scall1(open_bytes_input, bytes);
        let result = Scall1(load_from_port, bytes_input);
        result
    }
}

pub fn make_bytevector(data:&[u8]) -> ptr {
    unsafe {
        let ba = Smake_bytevector(data.len() as isize, 0);
        let dst = Sbytevector_data!(ba) as *mut u8;
        copy_nonoverlapping(data.as_ptr(), dst, data.len());
        ba
    }
}

pub fn from_bytevector(bytes : ptr) -> Vec<u8> {
    unsafe {
        let len = Sbytevector_length!(bytes) as usize;
        let data = Sbytevector_data!(bytes);
        let mut v:Vec<u8> = Vec::with_capacity(len);
        copy_nonoverlapping(data, v.as_mut_ptr(), len);
        v.set_len(len);
        v
    }
}

pub fn from_scheme_string(s : ptr) -> String {
    let string_to_utf8 = get_top_level_object(c"string->utf8");
    unsafe {
        let bytes = Scall1(string_to_utf8, s);
        let v = from_bytevector(bytes);
        String::from_utf8_unchecked(v)
    }
}