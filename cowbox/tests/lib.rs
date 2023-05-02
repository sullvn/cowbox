use cowbox::spawn;
use cowbox_testing::{run_test_rm, RmResult};
use std::io::Result;

#[test]
#[cfg(unix)]
fn sandboxed_rm() -> Result<()> {
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, tmp_dir| {
        let exit_code = spawn(tmp_dir, "rm", [file_path])?;
        Ok(exit_code == 0)
    })?;

    assert_eq!(rm_result, RmResult::NotRemoved);
    Ok(())
}

#[test]
#[cfg(windows)]
fn sandboxed_rm() -> Result<()> {
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, tmp_dir| {
        let exit_code = spawn(
            tmp_dir,
            "powershell",
            [
                "-Command",
                &format!("Remove-Item {}", file_path.as_os_str().to_string_lossy()),
            ],
        )?;
        Ok(exit_code == 0)
    })?;

    assert_eq!(rm_result, RmResult::NotRemoved);
    Ok(())
}
