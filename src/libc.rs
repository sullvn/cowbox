use std::ffi::{c_char, c_int};

extern "system" {
    pub fn unlink(_path: *const c_char) -> c_int;
    pub fn unlinkat(_dir_fd: c_int, _path: *const c_char, _flags: c_int) -> c_int;
}
