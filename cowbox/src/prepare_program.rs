use std::borrow::Cow;
use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;

#[cfg(not(target_os = "macos"))]
pub fn prepare_program<'a>(_injection_dir: &Path, program: &'a OsStr) -> Result<Cow<'a, OsStr>>
{
    Ok(Cow::Borrowed(program))
}

#[cfg(target_os = "macos")]
pub use macos::prepare_program;

mod macos {
    #![cfg(target_os = "macos")]

    use std::fs;
    use std::io::{Error, ErrorKind};
    use std::path::PathBuf;
    use which::which;

    use super::{Cow, OsStr, Path, Result};

    pub fn prepare_program<'a>(injection_dir: &Path, program: &'a OsStr) -> Result<Cow<'a, OsStr>>
    {
        let program_path = which(program).map_err(|e| {
            Error::new(ErrorKind::Other, e)
        })?;

        let copy_path: PathBuf = [injection_dir.as_ref(), program].iter().collect();
        fs::copy(program_path, &copy_path)?;

        Ok(Cow::Owned(copy_path.into_os_string()))
    }
}

