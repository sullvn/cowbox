use std::ffi::{c_char, c_int};

extern "system" {
    fn unlink(_path: *const c_char) -> c_int;
    fn unlinkat(_dir_fd: c_int, _path: *const c_char, _flags: c_int) -> c_int;
}

#[no_mangle]
pub extern "system" fn unlink_new(_path: *const c_char) -> c_int {
    0
}

#[no_mangle]
pub extern "system" fn unlinkat_new(_dir_fd: c_int, _path: *const c_char, _flags: c_int) -> c_int {
    0
}

#[repr(C)]
struct Interposed<T> {
    replacement: T,
    original: T,
}

#[used]
#[link_section = "__DATA,__interpose"]
static INTERPOSE_UNLINK: Interposed<unsafe extern "system" fn(*const c_char) -> c_int> =
    Interposed {
        replacement: unlink_new,
        original: unlink,
    };

#[used]
#[link_section = "__DATA,__interpose"]
static INTERPOSE_UNLINKAT: Interposed<
    unsafe extern "system" fn(c_int, *const c_char, c_int) -> c_int,
> = Interposed {
    replacement: unlinkat_new,
    original: unlinkat,
};
