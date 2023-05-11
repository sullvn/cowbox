use std::env;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

/// Home cache directory
///
/// Applications create a subfolder
/// for themselves.
///
#[cfg(unix)]
const HOME_CACHE_DIR: &str = ".cache";

/// Cache directory name
///
/// Example: `cowbox`
/// Example in a path: `~/.cache/cowbox`
///
const CACHE_DIR_NAME: &str = env!("CARGO_PKG_NAME");

///
/// Cache directory on Unix
///
/// - Adopt Linux practices on macOS. This
///   seems to be the de facto standard for
///   CLI tools
/// - Fallback to using `$HOME` as
///   `$XDG_CONFIG_HOME` isn't always
///   available
///
/// NOTE: The [`directories`][0] package is
/// not used as it has undesired behavior
/// for macOS. It seems to be made for macOS
/// applications, not CLI tools. As noted
/// above, the de facto standard is to [use
/// XDG for CLI tools][1].
///
/// [0]: https://github.com/dirs-dev/directories-rs
/// [1]: https://xdgbasedirectoryspecification.com
///
#[cfg(unix)]
pub fn cache_dir_path() -> Result<PathBuf> {
    if let Some(xdg_cache_home) = env::var_os("XDG_CACHE_HOME") {
        let path = [xdg_cache_home.as_os_str(), CACHE_DIR_NAME.as_ref()]
            .iter()
            .collect();
        return Ok(path);
    }

    let home = env::var_os("HOME").ok_or(Error::new(
        ErrorKind::NotFound,
        "HOME environment variable is not set",
    ))?;
    let path = [
        home.as_os_str(),
        HOME_CACHE_DIR.as_ref(),
        CACHE_DIR_NAME.as_ref(),
    ]
    .iter()
    .collect();

    Ok(path)
}

///
/// Cache directory on Windows
///
/// - Prefer %LOCALAPPDATA% as it is a standard
///   variable and is made for local
///   application data
/// - Don't prefer %APPDATA% as roaming support
///   is not desirable. The cache has machine
///   specific data such as binaries, paths,
///   or DLLs for the specific OS and
///   architecture.
/// - Don't fallback to anything, such as
///   %USERPROFILE%. This simplifies behavior.
///
#[cfg(windows)]
pub fn cache_dir_path() -> Result<PathBuf> {
    let local_app_data = env::var_os("LOCALAPPDATA").ok_or(Error::new(
        ErrorKind::NotFound,
        "LOCALAPPDATA environment variable is not set",
    ))?;
    let path = [
        local_app_data.as_os_str(),
        CACHE_DIR_NAME.as_ref(),
        "cache".as_ref(),
    ]
    .iter()
    .collect();

    Ok(path)
}
