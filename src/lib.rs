use std::ffi::{c_char, c_int};

#[no_mangle]
pub extern "system" fn unlink(_path: *const c_char) -> c_int {
    0
}

#[no_mangle]
pub extern "system" fn unlinkat(_dir_fd: c_int, _path: *const c_char, _flags: c_int) -> c_int {
    0
}
