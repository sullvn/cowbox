use core::ffi::{c_int, c_long, c_void};
use windows_sys::core::PSTR;
use windows_sys::Win32::Foundation::{BOOL, HANDLE, HINSTANCE, HWND};

extern "system" {
    pub fn DetourAttach(target_fn: &mut *const c_void, detour_fn: *const c_void) -> c_long;
    pub fn DetourDetach(target_fn: &mut *const c_void, detour_fn: *const c_void) -> c_long;
    pub fn DetourUpdateThread(thread: HANDLE) -> c_long;

    pub fn DetourTransactionBegin() -> c_long;
    pub fn DetourTransactionCommit() -> c_long;

    pub fn DetourIsHelperProcess() -> BOOL;
    pub fn DetourRestoreAfterWith() -> BOOL;

    #[allow(dead_code)]
    pub fn DetourFinishHelperProcess(
        window_handle: HWND,
        dll_handle: HINSTANCE,
        lpstr: PSTR,
        int: c_int,
    );
}
