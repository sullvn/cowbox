use std::ffi::OsStr;
use std::io::{Error, ErrorKind, Result};

/// Trait for types which can be coerced,
/// maybe, into a string slice.
///
/// Essentially a specific implementation
/// of [`std::convert::TryFrom`]. A new
/// trait is required due to orphaning
/// rules.
///
/// TODO: Replace with `TryFrom` on
/// custom wrapper types.
///
pub trait AsStr {
    fn as_str(&self, description: &str) -> Result<&str>;
}

impl<T: AsRef<OsStr>> AsStr for T {
    fn as_str(&self, description: &str) -> Result<&str> {
        self.as_ref().to_str().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("{description} is invalid UTF-8"),
            )
        })
    }
}
