#![cfg(windows)]

use std::io::{Error, Result};
use windows_sys::Win32::Foundation::{FALSE, WAIT_FAILED, WIN32_ERROR};

/// Convert Win32 API status return
/// values into informative `Result`
pub trait StatusBool {
    fn ok(&self) -> Result<()>;
}

impl StatusBool for i32 {
    fn ok(&self) -> Result<()> {
        // Checking for `FALSE` is stronger
        // than checking for `TRUE`. Technically
        // any non-zero value should be treated
        // as `true`.
        if *self == FALSE {
            return Err(Error::last_os_error());
        }

        Ok(())
    }
}

impl StatusBool for WIN32_ERROR {
    fn ok(&self) -> Result<()> {
        if *self == WAIT_FAILED {
            return Err(Error::last_os_error());
        }

        Ok(())
    }
}
