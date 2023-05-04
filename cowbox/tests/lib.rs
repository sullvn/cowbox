use cowbox::spawn;
use cowbox_testing::{run_test_rm, RmResult, TempDir, TempPath};
use std::io::Result;

use cowbox::INJECTION_BINARIES;

#[test]
fn sandboxed_rm() -> Result<()> {
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, tmp_dir| {
        let exit_code = spawn_rm(file_path, tmp_dir)?;
        Ok(exit_code == 0)
    })?;

    assert_eq!(rm_result, RmResult::NotRemoved);
    Ok(())
}

#[test]
fn binary_hashes() -> Result<()> {
    run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, tmp_dir| {
        spawn_rm(file_path, tmp_dir)?;

        for ib in INJECTION_BINARIES.iter() {
            assert_eq!(ib.hash(), ib.read_hash(tmp_dir)?);
            assert_ne!(ib.hash(), 0);
        }

        Ok(true)
    })?;

    Ok(())
}

#[cfg(unix)]
fn spawn_rm(file_path: &TempPath, tmp_dir: &TempDir) -> Result<i32> {
    spawn(tmp_dir, "rm", [file_path])
}

#[cfg(windows)]
fn spawn_rm(file_path: &TempPath, tmp_dir: &TempDir) -> Result<i32> {
    spawn(
        tmp_dir,
        "powershell",
        [
            "-Command",
            &format!("Remove-Item {}", file_path.as_os_str().to_string),
        ],
    )
}
