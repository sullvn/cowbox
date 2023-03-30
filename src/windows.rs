use core::ffi::{c_ulong, c_void};
use windows_sys::core::{PCSTR, PCWSTR};
use windows_sys::Win32::Foundation::{BOOL, HINSTANCE, TRUE, FALSE};
use windows_sys::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows_sys::Win32::System::Threading::GetCurrentThread;
use windows_sys::Win32::Storage::FileSystem::{DeleteFileA, DeleteFileW};

use crate::detours::{
    DetourAttach,
    DetourDetach,
    DetourIsHelperProcess,
    DetourRestoreAfterWith,
    DetourTransactionBegin,
    DetourTransactionCommit,
    DetourUpdateThread,
};

#[no_mangle]
pub unsafe extern "system" fn DeleteFileA_new(_path: PCSTR) -> BOOL {
    0
}

#[no_mangle]
pub unsafe extern "system" fn DeleteFileW_new(_path: PCWSTR) -> BOOL {
    0
}

///
/// https://learn.microsoft.com/en-us/cpp/build/run-time-library-behavior
///
#[no_mangle]
pub unsafe extern "system" DllMain(
    dll_handle: HINSTANCE,
    reason: c_ulong,
    reserved: *mut c_void,
) -> BOOL {
    if (DetourIsHelperThread()) {
        return TRUE;
    }

    let DeleteFileA_variable = DeleteFileA;
    let DeleteFileW_variable = DeleteFileW;

    if (reason == DLL_PROCESS_ATTACH) {
        DetourRestoreAfterWith();

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&DeleteFileA_variable, DelteFileA_new);
        DetourAttach(&DeleteFileW_variable, DelteFileW_new);
        DetourTransactionCommit();
    } else if (reason == DLL_PROCESS_DETACH){
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourDetach(&DeleteFileA_variable, DelteFileA_new);
        DetourDetach(&DeleteFileW_variable, DelteFileW_new);
        DetourTransactionCommit();
    }


    return TRUE;
}
