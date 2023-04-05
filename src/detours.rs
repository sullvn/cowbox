use core::ffi::{c_int, c_long, c_void};
use windows_sys::core::PSTR;
use windows_sys::Win32::Foundation::{BOOL, HANDLE, HINSTANCE, HWND};
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
use windows_sys::Win32::System::Threading::{
    PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTUPINFOA,
};

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

type DetourCreateProcessRoutineA = extern "system" fn(
    application_name: PCSTR,
    command_line: PSTR,
    process_attributes: *const SECURITY_ATTRIBUTES,
    thread_attributes: *const SECURITY_ATTRIBUTES,
    inherit_handles: BOOL,
    creation_flags: PROCESS_CREATION_FLAGS,
    environment: *const c_void,
    current_directory: PCSTR,
    startup_info: *const STARTUPINFOA,
    process_information: *mut PROCESS_INFORMATION,
) -> BOOL;

extern "system" {
    pub fn DetourCreateProcessWithDllEx(
        application_name: PCSTR,
        command_line: PSTR,
        process_attributes: *const SECURITY_ATTRIBUTES,
        thread_attributes: *const SECURITY_ATTRIBUTES,
        inherit_handles: BOOL,
        creation_flags: PROCESS_CREATION_FLAGS,
        environment: *const c_void,
        current_directory: PCSTR,
        startup_info: *const STARTUPINFOA,
        process_information: *mut PROCESS_INFORMATION,
        dll_name: PCSTR,
        create_process_fn: Option<DetourCreateProcessRoutineA>,
    ) -> BOOL;
}
