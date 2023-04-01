// use libc::{dlsym, RTLD_NEXT};
use std::ffi::{c_char, c_int};

#[no_mangle]
extern "C" fn unlink(_path: *const c_char) -> c_int {
    0
}

#[no_mangle]
extern "C" fn unlinkat(_dir_fd: c_int, _path: *const c_char, _flags: c_int) -> c_int {
    0
}
