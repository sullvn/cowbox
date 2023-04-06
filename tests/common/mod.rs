use std::fs::File;
use std::io::Result;
use tempfile::{NamedTempFile, TempDir, TempPath};

pub enum RmResult {
    Removed,
    NotRemoved,
}

pub fn run_test_rm<F>(result: RmResult, rm_fn: F) -> Result<()>
where
    F: FnOnce(&TempPath, &TempDir) -> Result<()>,
{
    let cargo_tmp_dir_path = env!("CARGO_TARGET_TMPDIR");
    let tmp_dir_path = TempDir::new_in(cargo_tmp_dir_path)?;
    let rm_file_path = NamedTempFile::new_in(cargo_tmp_dir_path)?.into_temp_path();

    assert!(
        File::open(&rm_file_path).is_ok(),
        "test file wasn't created"
    );

    rm_fn(&rm_file_path, &tmp_dir_path)?;

    match result {
        RmResult::NotRemoved => {
            assert!(
                File::open(&rm_file_path).is_ok(),
                "test file was incorrectly removed"
            );
        }
        RmResult::Removed => {
            assert!(
                File::open(&rm_file_path).is_err(),
                "test file incorrectly still exists"
            );
        }
    }

    Ok(())
}
