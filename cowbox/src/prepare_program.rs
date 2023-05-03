use std::borrow::Cow;
use std::ffi::OsStr;
use std::io::Result;
use std::path::Path;

#[cfg(not(target_os = "macos"))]
pub fn prepare_program<'a>(_injection_dir: &Path, program: &'a OsStr) -> Result<Cow<'a, OsStr>> {
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

    /// Prepare macOS program for injection
    ///
    /// macOS has ["System Integrity Protection"][0],
    /// or "SIP", which prevents dynamic libraries
    /// from being injected (interposed) into a
    /// secured program.
    ///
    /// Unlucky for `cowbox`, all(?) included
    /// system binaries are secured by SIP. This
    /// makes `cowbox` much less useful by default,
    /// as it can't sandbox something as rudimentary
    /// as `rm`.
    ///
    /// The workaround is to make a user copy of
    /// these binaries and strip the SIP protections.
    /// [Here's a blog post for inspiration][1].
    ///
    /// WARNING: This function, doing I/O, is
    /// probably a slow part of the program. Work
    /// should be done to make this as fast as
    /// possible.
    ///
    /// [0]: https://developer.apple.com/library/archive/documentation/Security/Conceptual/System_Integrity_Protection_Guide/Introduction/Introduction.html
    /// [1]: https://metalbear.co/blog/fun-with-macoss-sip/
    ///
    pub fn prepare_program<'a>(injection_dir: &Path, program: &'a OsStr) -> Result<Cow<'a, OsStr>> {
        let program_path = which(program).map_err(|e| Error::new(ErrorKind::Other, e))?;

        // TODO: Detect SIP on program here and
        // return, doing nothing, if there are
        // no protections to remove.
        let copy_path: PathBuf = [injection_dir.as_ref(), program].iter().collect();

        // TODO: Use reflink (copy-on-write file) if
        // available (on APFS; not HFS+). This avoids
        // unnecessary copying, which is probably
        // the bulk of the I/O in this function.
        fs::copy(program_path, &copy_path)?;

        Ok(Cow::Owned(copy_path.into_os_string()))
    }
}
