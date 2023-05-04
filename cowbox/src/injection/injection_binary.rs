use std::ffi::OsStr;
use std::fs::{self, create_dir_all};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use xxhash_rust::const_xxh3::xxh3_64;

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
    hash: u64,
}

impl InjectionBinary {
    pub const fn new(file_name: &'static str, bytes: &'static [u8]) -> Self {
        let hash = xxh3_64(bytes);

        Self {
            bytes,
            file_name,
            hash,
        }
    }

    pub fn hash(&self) -> u64 {
        self.hash
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
        if let Ok(true) = self.exists(&dir) {
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
        dir.as_ref().join(self.file_name)
    }

    /// Hash file path
    ///
    /// NOTE: See `create` documentation for
    /// this could evolve.
    ///
    pub fn hash_path<P: AsRef<Path>>(&self, dir: P) -> PathBuf {
        let mut bp = self.binary_path(&dir);
        let extension = bp.extension().unwrap_or(OsStr::new(""));

        bp.set_extension([extension, OsStr::new("hash")].join(OsStr::new(".")));
        bp
    }

    pub fn read_hash<P: AsRef<Path>>(&self, dir: P) -> Result<u64> {
        self.binary_path(&dir).try_exists()?;

        let hash_bytes_slice = fs::read(self.hash_path(&dir))?;
        let hash_bytes_array: [u8; 8] = hash_bytes_slice.try_into().map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                "File has injection binary hash of wrong length",
            )
        })?;

        // Use native byte order as portability is
        // not necessary. Injection binary and its
        // hash should stay local to one system
        // anyways.
        let found_hash = u64::from_ne_bytes(hash_bytes_array);
        Ok(found_hash)
    }

    /// Check if everything exists correctly
    /// on the file system.
    ///
    /// The expectation is that this will be
    /// true 99% of the time, besides on
    /// the first run and any subsequent
    /// updates of `cowbox`.
    ///
    fn exists<P: AsRef<Path>>(&self, dir: P) -> Result<bool> {
        let hash_matches = self.read_hash(dir)? == self.hash;

        Ok(hash_matches)
    }

    /// Create injection binary and metadata
    /// (eg. hash) on the filesystem
    ///
    /// QUESTION: Is there a way to make this
    /// update process atomic? Does it even
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
        create_dir_all(&dir)?;
        fs::write(self.binary_path(&dir), self.bytes)?;

        // See note above about byte order
        fs::write(self.hash_path(&dir), self.hash.to_ne_bytes())?;

        Ok(())
    }
}
