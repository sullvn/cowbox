mod injection;
mod spawn_process;

use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;

use injection::INJECTION_BINARIES;
use spawn_process::spawn_process;

pub fn spawn<P, S, T, A>(injection_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    INJECTION_BINARIES.update(&injection_dir)?;
    spawn_process(injection_dir, program, args)
}
