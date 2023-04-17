use std::fs::File;
use std::path::Path;
use std::io::Result;
use tempfile::{NamedTempFile, TempDir, TempPath};

#[derive(Debug, PartialEq, Eq)]
pub enum RmResult {
    Removed,
    NotRemoved,
}

pub fn run_test_rm<P, F>(cargo_tmp_dir: P, rm_fn: F) -> Result<RmResult>
where
    P: AsRef<Path>,
    F: FnOnce(&TempPath, &TempDir) -> Result<bool>,
{
    let tmp_dir_path = TempDir::new_in(&cargo_tmp_dir)?;
    let rm_file_path = NamedTempFile::new_in(&cargo_tmp_dir)?.into_temp_path();

    assert!(
        File::open(&rm_file_path).is_ok(),
        "test file wasn't created"
    );

    let success = rm_fn(&rm_file_path, &tmp_dir_path)?;
    assert!(success, "rm program unexpectedly failed");

    let result = match File::open(&rm_file_path) {
        Ok(_) => RmResult::NotRemoved,
        Err(_) => RmResult::Removed,
    };
    Ok(result)
}
