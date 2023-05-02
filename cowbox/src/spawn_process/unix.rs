#![cfg(unix)]

use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;
use std::process::Command;

use crate::injection::constants::INJECTION_ENV_KEY;
use crate::injection::INJECTION_BINARIES;

/// Spawn process with injection on Unix
///
/// Both macOS and Linux allow dynamic
/// library injection via an environment
/// variable, which is read by the
/// runtime linker.
///
/// TODO: Split up this function once
/// the Linux implementation evolves
/// away from dynamic library injection.
/// This will probably be something
/// like detouring syscalls via `ptrace`.
///
///
/// ## Assorted Reading
///
/// - [Intercepting and Emulating Linux
///   System Calls with Ptrace][0]
/// - [Modifying System Call Arguments
///   With ptrace][1]
/// - [Emulating Windows system calls in
///   Linux][2]
/// - [Emulating Windows system calls in
///   Linux, take 2][3]
/// - [Replacing ptrace() on macOS][4]
///
/// [0]: https://nullprogram.com/blog/2018/06/23/
/// [1]: https://www.alfonsobeato.net/c/modifying-system-call-arguments-with-ptrace/
/// [2]: https://lwn.net/Articles/824380/
/// [3]: https://lwn.net/Articles/826313/
/// [4]: http://uninformed.org/index.cgi?v=4&a=3&p=14
///
pub fn spawn_process<P, S, T, A>(injection_dir: P, program: S, args: A) -> Result<i32>
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
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Process quit early",
        ))?;

    Ok(exit_code)
}
