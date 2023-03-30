use core::ffi::{c_long, c_void, c_int, c_bool};
use windows_sys::Win32::Foundation::{HANDLE, HWND, HINSTANCE, PSTR};

extern "C" {
    pub unsafe fn DetourAttach(target_fn: **mut c_void, detour_fn: *const c_void) -> c_long;
    pub unsafe fn DetourDetach(target_fn: **mut c_void, detour_fn: *const c_void) -> c_long;
    pub unsafe fn DetourUpdateThread(thread: HANDLE) -> c_long;

    pub unsafe fn DetourTransactionBegin() -> c_long;
    pub unsafe fn DetourTransactionCommit() -> c_long;

    pub unsafe fn DetourIsHelperProcess() -> c_bool;
    pub unsafe fn DetourRestoreAfterWith() -> c_bool;
}

extern "system" {
    pub unsafe fn DetourFinishHelperProcess(
        window_handle: HWND,
        dll_handle: HINSTANCE,
        lpstr: PSTR,
        int: c_int,
    );
}
