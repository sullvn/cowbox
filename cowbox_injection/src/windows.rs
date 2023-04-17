use core::ffi::{c_ulong, c_void};
use detours_sys::{
    DetourAttach, DetourDetach, DetourIsHelperProcess, DetourRestoreAfterWith,
    DetourTransactionBegin, DetourTransactionCommit, DetourUpdateThread,
};
use windows_sys::core::{PCSTR, PCWSTR};
use windows_sys::Win32::Foundation::{BOOL, HMODULE, TRUE};
use windows_sys::Win32::Storage::FileSystem::{DeleteFileA, DeleteFileW};
use windows_sys::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows_sys::Win32::System::Threading::GetCurrentThread;

#[allow(non_snake_case)]
extern "system" fn DeleteFileAHook(_path: PCSTR) -> BOOL {
    TRUE
}

#[allow(non_snake_case)]
extern "system" fn DeleteFileWHook(_path: PCWSTR) -> BOOL {
    TRUE
}

///
/// https://learn.microsoft.com/en-us/cpp/build/run-time-library-behavior
///
#[no_mangle]
unsafe extern "system" fn DllMain(
    _dll_handle: HMODULE,
    reason: c_ulong,
    _reserved: *mut c_void,
) -> BOOL {
    if DetourIsHelperProcess() == TRUE {
        return TRUE;
    }

    #[allow(non_snake_case)]
    let mut DeleteFileAPointer = DeleteFileA as *const c_void;

    #[allow(non_snake_case)]
    let mut DeleteFileWPointer = DeleteFileW as *const c_void;

    if reason == DLL_PROCESS_ATTACH {
        DetourRestoreAfterWith();

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&mut DeleteFileAPointer, DeleteFileAHook as *const c_void);
        DetourAttach(&mut DeleteFileWPointer, DeleteFileWHook as *const c_void);
        DetourTransactionCommit();
    } else if reason == DLL_PROCESS_DETACH {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourDetach(&mut DeleteFileAPointer, DeleteFileAHook as *const c_void);
        DetourDetach(&mut DeleteFileWPointer, DeleteFileWHook as *const c_void);
        DetourTransactionCommit();
    }

    TRUE
}
