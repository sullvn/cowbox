use std::fs::File;
use std::io::Result;
use tempfile::{NamedTempFile, TempDir, TempPath};

#[derive(Debug, PartialEq, Eq)]
pub enum RmResult {
    Removed,
    NotRemoved,
}

pub fn run_test_rm<F>(rm_fn: F) -> Result<RmResult>
where
    F: FnOnce(&TempPath, &TempDir) -> Result<bool>,
{
    let cargo_tmp_dir_path = env!("CARGO_TARGET_TMPDIR");
    let tmp_dir_path = TempDir::new_in(cargo_tmp_dir_path)?;
    let rm_file_path = NamedTempFile::new_in(cargo_tmp_dir_path)?.into_temp_path();

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
