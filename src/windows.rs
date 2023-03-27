use windows_sys::core::{PCSTR, PCWSTR};
use windows_sys::Win32::Foundation::BOOL;

#[no_mangle]
pub unsafe extern "system" fn DeleteFileA(_path: PCSTR) -> BOOL {
    0
}

#[no_mangle]
pub unsafe extern "system" fn DeleteFileW(_path: PCWSTR) -> BOOL {
    0
}
