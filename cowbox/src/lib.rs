mod injection;

use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;
use std::process::Command;
use injection::INJECTION_BINARIES;

#[cfg(target_os = "linux")]
const INJECTION_ENV_KEY: &str = "LD_PRELOAD";
#[cfg(target_os = "macos")]
const INJECTION_ENV_KEY: &str = "DYLD_INSERT_LIBRARIES";

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn spawn_injected_process<P, S, T, A>(injection_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    let injection_binary_path = INJECTION_BINARIES.preferred().binary_path(&injection_dir);
    let exit_code = Command::new(program)
        .args(args)
        .env(INJECTION_ENV_KEY, injection_binary_path)
        .status()?
        .code()
        .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Process quit early"))?;

    Ok(exit_code)
}

#[cfg(target_os = "windows")]
fn spawn_injected_process<P, S, T, A>(injection_dir: P, program: S, args: A) -> Result<i32>
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

pub fn spawn<P, S, T, A>(injection_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    INJECTION_BINARIES.update(&injection_dir)?;
    spawn_injected_process(injection_dir, program, args)
}
