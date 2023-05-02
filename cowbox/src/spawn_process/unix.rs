#![cfg(unix)]

use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;
use std::process::Command;

use crate::injection::constants::INJECTION_ENV_KEY;
use crate::injection::INJECTION_BINARIES;

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
