//! Convert C format strings to Rust.

extern crate libc;

use libc::size_t;
use std::os::raw::*;

pub unsafe fn vsprintf_raw<V>(buffer: &mut [u8], format: *const c_char, va_list: *mut V) -> i32 {
    vsnprintf_wrapper(
        buffer.as_mut_ptr(),
        buffer.len(),
        format,
        va_list as *mut c_void,
    ) as i32
}

extern "C" {
    fn vsnprintf_wrapper(
        buffer: *mut u8,
        size: size_t,
        format: *const c_char,
        va_list: *mut c_void,
    ) -> libc::c_int;
}
