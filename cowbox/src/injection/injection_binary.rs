use std::ffi::OsStr;
use std::fs::{self, create_dir_all, read_to_string};
use std::io::Result;
use std::path::{Path, PathBuf};

pub struct InjectionBinary {
    bytes: &'static [u8],
    file_name: &'static str,
    hash: u128,
}

impl InjectionBinary {
    pub const fn new(file_name: &'static str, bytes: &'static [u8]) -> Self {
        Self {
            bytes,
            file_name,
            hash: 0,
        }
    }

    pub fn update<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        if let Some(true) = self.exists(&dir) {
            return Ok(());
        }

        self.create(&dir)
    }

    fn exists<P: AsRef<Path>>(&self, dir: P) -> Option<bool> {
        self.binary_path(&dir).try_exists().ok()?;

        let found_hash: u128 = read_to_string(self.hash_path(&dir)).ok()?.parse().ok()?;
        let hash_matches = found_hash == self.hash;

        Some(hash_matches)
    }

    fn create<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        let hash_str = format!("{:x}", self.hash);

        create_dir_all(&dir)?;
        fs::write(self.binary_path(&dir), self.bytes)?;
        fs::write(self.hash_path(&dir), hash_str.as_bytes())?;

        Ok(())
    }

    fn binary_path<P: AsRef<Path>>(&self, dir: P) -> PathBuf {
        [dir.as_ref(), self.file_name.as_ref()].iter().collect()
    }

    fn hash_path<P: AsRef<Path>>(&self, dir: P) -> PathBuf {
        let mut bp = self.binary_path(&dir);
        let extension = bp.extension().unwrap_or(OsStr::new(""));

        bp.set_extension([extension, OsStr::new("hash")].join(OsStr::new(".")));
        bp
    }
}