mod as_str;
mod injection;
mod prepare_program;
mod spawn_process;

use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;

// TODO: Only export `INJECTION_BINARIES` for tests
pub use injection::INJECTION_BINARIES;
use prepare_program::prepare_program;
use spawn_process::spawn_process;

/// Spawn process with sandboxed filesystem access
///
/// Provide the path of `injection_dir` which
/// `spawn` can use to setup helper files.
///
pub fn spawn<P, S, T, A>(injection_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    INJECTION_BINARIES.update(&injection_dir)?;
    let program = prepare_program(injection_dir.as_ref(), program.as_ref())?;
    spawn_process(injection_dir, program, args)
}
