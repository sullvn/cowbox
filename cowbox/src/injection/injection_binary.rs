use std::ffi::OsStr;
use std::fs::{self, create_dir_all, read_to_string};
use std::io::Result;
use std::path::{Path, PathBuf};

/// Injection binary definition
///
/// The binary, expected to be a dynamic
/// linked library, is loaded into the
/// target program on creation. Standard
/// OS calls in the program are detoured
/// through cowbox's middleware.
///
pub struct InjectionBinary {
    ///
    /// Payload of binary
    ///
    /// Statically linked into `cowbox`
    /// for easy deployment. The
    /// payload, a dynamic linked
    /// library, is written to file as
    /// required.
    ///
    bytes: &'static [u8],

    /// Filename of binary when it is
    /// deployed onto the filesystem
    file_name: &'static str,

    /// Hash of binary used for
    /// invalidation of any injection
    /// binary already deployed onto
    /// the file system
    hash: u128,
}

impl InjectionBinary {
    pub const fn new(file_name: &'static str, bytes: &'static [u8]) -> Self {
        Self {
            bytes,
            file_name,
            // TODO: Actually compute hash
            hash: 0,
        }
    }

    /// Update binary on filesystem
    ///
    /// Should be done on every instantiation
    /// of `cowbox` so the latest payload is
    /// used.
    ///
    /// WARNING: This process is probably the
    /// slowest part of `cowbox`. Avoid as
    /// many I/O calls as possible.
    ///
    pub fn update<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        if let Some(true) = self.exists(&dir) {
            return Ok(());
        }

        self.create(&dir)
    }

    /// Binary payload path
    ///
    /// The standard OS-specific file name
    /// is used, placed within the given
    /// directory.
    ///
    pub fn binary_path<P: AsRef<Path>>(&self, dir: P) -> PathBuf {
        [dir.as_ref(), self.file_name.as_ref()].iter().collect()
    }

    /// Check if everything exists correctly
    /// on the file system.
    ///
    /// The expectation is that this will be
    /// true 99% of the time, besides on
    /// the first run and any subsequent
    /// updates of `cowbox`.
    ///
    fn exists<P: AsRef<Path>>(&self, dir: P) -> Option<bool> {
        self.binary_path(&dir).try_exists().ok()?;

        let found_hash: u128 = read_to_string(self.hash_path(&dir)).ok()?.parse().ok()?;
        let hash_matches = found_hash == self.hash;

        Some(hash_matches)
    }

    /// Create injection binary and metadata
    /// (eg. hash) on the filesystem
    ///
    /// QUESTION: Is there a way to make this
    /// update process atomic. Does it even
    /// matter that much?
    ///
    /// Example: Binary could have the hash
    /// as its file name. A symlink, with the
    /// standard "nice" dylib name, could
    /// link to the hash. Updating the symlink
    /// location is an atomic operation.
    ///
    /// Problem: For some reason, symlinks
    /// [require administrator privileges on
    /// Windows][0].
    ///
    /// [0]: https://learn.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/create-symbolic-links#default-values
    ///
    fn create<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        let hash_str = format!("{:x}", self.hash);

        create_dir_all(&dir)?;
        fs::write(self.binary_path(&dir), self.bytes)?;
        fs::write(self.hash_path(&dir), hash_str.as_bytes())?;

        Ok(())
    }

    /// Hash file path
    ///
    /// NOTE: See `create` documentation for
    /// this could evolve.
    ///
    fn hash_path<P: AsRef<Path>>(&self, dir: P) -> PathBuf {
        let mut bp = self.binary_path(&dir);
        let extension = bp.extension().unwrap_or(OsStr::new(""));

        bp.set_extension([extension, OsStr::new("hash")].join(OsStr::new(".")));
        bp
    }
}
