#![cfg(windows)]

use detours_sys::DetourCreateProcessWithDllExA;
use std::ffi::{CString, OsStr};
use std::io::Result;
use std::mem::zeroed;
use std::path::Path;
use std::ptr;
use windows_sys::Win32::Foundation::{CloseHandle, FALSE, TRUE};
use windows_sys::Win32::System::Threading::{
    GetExitCodeProcess, WaitForSingleObject, INFINITE, PROCESS_INFORMATION, STARTUPINFOA,
};

use crate::INJECTION_BINARIES;

type ExitCode = u32;

pub fn spawn_process<P, S, T, A>(injection_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    let mut exit_code: ExitCode = 0;

    // TODO: Check CString is the right format
    let program_cstr = CString::new(program.as_ref().to_string_lossy().as_ref())?;
    let args_cstr = CString::new(
        args.into_iter()
            // TODO: To lossy?
            .map(|a| format!("\"{}\"", a.as_ref().to_string_lossy()))
            .collect::<Vec<String>>()
            .join(" "),
    )?;
    // TODO: To lossy?
    // TODO: Don't create program_cstr and args_cstr before?
    let combined_cstr =
        CString::new([program_cstr.to_string_lossy(), args_cstr.to_string_lossy()].join(" "))?;
    let dll_path = INJECTION_BINARIES.preferred().binary_path(injection_dir);
    let dll_path_cstr = CString::new(dll_path.into_os_string().to_string_lossy().as_ref())?;

    unsafe {
        let si: STARTUPINFOA = zeroed();
        let mut pi: PROCESS_INFORMATION = zeroed();
        let result = DetourCreateProcessWithDllExA(
            ptr::null(),
            combined_cstr.into_bytes_with_nul().as_mut_ptr(),
            ptr::null(),
            ptr::null(),
            FALSE,
            0,
            ptr::null(),
            ptr::null(),
            &si,
            &mut pi,
            dll_path_cstr.into_bytes_with_nul().as_ptr(),
            None,
        );
        assert_eq!(result, TRUE, "process couldn't be created executed");

        WaitForSingleObject(pi.hProcess, INFINITE);

        let result = GetExitCodeProcess(pi.hProcess, &mut exit_code);
        assert_eq!(result, TRUE, "exit code couldn't be retrieved");

        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
    }

    Ok(exit_code as i32)
}
