#![cfg(windows)]

use detours_sys::DetourCreateProcessWithDllExA;
use std::borrow::Cow;
use std::ffi::{CString, OsStr};
use std::io::{Error, ErrorKind, Result};
use std::mem::zeroed;
use std::path::Path;
use std::ptr;
use windows_sys::Win32::Foundation::{CloseHandle, FALSE};
use windows_sys::Win32::System::Threading::{
    GetExitCodeProcess, WaitForSingleObject, INFINITE, PROCESS_INFORMATION, STARTUPINFOA,
};

use super::status_bool::StatusBool;
use crate::as_str::AsStr;
use crate::INJECTION_BINARIES;

/// Exit code as returned by
/// [`GetExitCodeProcess`][0]
///
/// [0]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess
///
type ExitCode = u32;

/// Spawn process with injection on Windows
///
/// Windows requires(?) the [Detours][0]
/// library to inject code into a process.
///
/// This is contrast to macOS and Linux,
/// where a built-in environment variable
/// does the trick.
///
/// [0]: https://github.com/microsoft/Detours
///
pub fn spawn_process<P, S, T, A>(injection_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    let mut exit_code: ExitCode = 0;

    // Avoid as many intermediate allocations as
    // possible. Iterators are not used due to
    // overcomplication, such as from [creating
    // borrowed references][0].
    //
    // A reasonable default buffer size is used which
    // can hold 99% of shell commands.
    //
    // [0]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#what-are-gats
    //
    let mut program_args = String::with_capacity(512);
    program_args.push('"');
    program_args.push_str(escape_quotes_program(program.as_str("program name")?)?);
    program_args.push('"');
    for a in args {
        program_args.push_str(" \"");
        program_args.push_str(&escape_quotes_arg(a.as_str("program argument")?));
        program_args.push('"');
    }
    let program_args_cstr = CString::new(program_args)?;

    let dll_path = INJECTION_BINARIES.preferred().binary_path(injection_dir);
    let dll_path_cstr = CString::new(dll_path.as_str("DLL path")?)?;

    unsafe {
        let si: STARTUPINFOA = zeroed();
        let mut pi: PROCESS_INFORMATION = zeroed();

        DetourCreateProcessWithDllExA(
            ptr::null(),
            program_args_cstr.into_bytes_with_nul().as_mut_ptr(),
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
        )
        .ok()?;

        WaitForSingleObject(pi.hProcess, INFINITE).ok()?;
        GetExitCodeProcess(pi.hProcess, &mut exit_code).ok()?;
        CloseHandle(pi.hProcess).ok()?;
        CloseHandle(pi.hThread).ok()?;
    }

    Ok(exit_code as i32)
}

/// Sanitize program name for Windows process API
///
/// Seems like double quotes are not allowed, but
/// the documentation does not say so explicitly.
///
/// It just says:
///
/// > The first argument (argv[0]) is treated
/// > specially. It represents the program name.
/// > [...] The later rules in this list don't
/// > apply.
///
/// See:
/// https://learn.microsoft.com/en-us/cpp/c-language/parsing-c-command-line-arguments
///
fn escape_quotes_program(program: &str) -> Result<&str> {
    if program.contains('"') {
        return Err(Error::new(
            ErrorKind::Other,
            "program name cannot contain double quotes",
        ));
    }

    Ok(program)
}

/// Sanitize program argument for Windows process API
///
/// See:
/// https://learn.microsoft.com/en-us/cpp/c-language/parsing-c-command-line-arguments
///
fn escape_quotes_arg(arg: &str) -> Cow<str> {
    if arg.contains('"') {
        return Cow::Owned(arg.replace('"', "\"\""));
    }

    Cow::Borrowed(arg)
}
