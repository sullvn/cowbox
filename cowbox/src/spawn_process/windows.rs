#![cfg(windows)]

use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;

pub fn spawn_process<P, S, T, A>(injection_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    unsafe {
        let si: STARTUPINFOA = zeroed();
        let mut pi: PROCESS_INFORMATION = zeroed();
        let result = DetourCreateProcessWithDllExA(
            ptr::null(), // TODO: use program
            rm_cmd.into_bytes_with_nul().as_mut_ptr(), // TODO: use args
            ptr::null(),
            ptr::null(),
            FALSE,
            0,
            ptr::null(),
            ptr::null(),
            si,
            pi,
            CString::new(dll_path)
                .unwrap()
                .into_bytes_with_nul()
                .as_ptr(),
            None,
        );
        assert_eq!(result, TRUE, "process couldn't be created executed");

        WaitForSingleObject(pi.hProcess, INFINITE);

        let result = GetExitCodeProcess(pi.hProcess, &mut exit_code);
        assert_eq!(result, TRUE, "exit code couldn't be retrieved");

        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
    }
}

