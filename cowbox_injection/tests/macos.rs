#![cfg(target_os = "macos")]

use cowbox_testing::{run_test_rm, RmResult};
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn normal_rm() -> Result<()> {
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, _| {
        Ok(Command::new("rm")
            .arg(file_path)
            .env_clear()
            .status()?
            .success())
    })?;

    assert_eq!(rm_result, RmResult::Removed);
    Ok(())
}

#[test]
fn sandboxed_sip_rm() -> Result<()> {
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, _| {
        Ok(Command::new("rm")
            .arg(file_path)
            .env_clear()
            .env(
                "DYLD_INSERT_LIBRARIES",
                "../target/release/libcowbox_injection.dylib",
            )
            .status()?
            .success())
    })?;

    assert_eq!(rm_result, RmResult::Removed);
    Ok(())
}

#[test]
fn sandboxed_rm() -> Result<()> {
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, tmp_dir_path| {
        let rm_copy_path: PathBuf = [tmp_dir_path.as_ref(), "rm".as_ref()].iter().collect();
        fs::copy("/bin/rm", &rm_copy_path)?;

        Ok(Command::new(rm_copy_path)
            .arg(file_path)
            .env_clear()
            .env(
                "DYLD_INSERT_LIBRARIES",
                "../target/release/libcowbox_injection.dylib",
            )
            .status()?
            .success())
    })?;

    assert_eq!(rm_result, RmResult::NotRemoved);
    Ok(())
}

#[test]
fn missing_dylib_rm() -> Result<()> {
    let mut exit_code: Option<i32> = None;
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, tmp_dir_path| {
        let rm_copy_path: PathBuf = [tmp_dir_path.as_ref(), "rm".as_ref()].iter().collect();
        fs::copy("/bin/rm", &rm_copy_path)?;

        exit_code = Command::new(rm_copy_path)
            .arg(file_path)
            .env_clear()
            .env("DYLD_INSERT_LIBRARIES", "../target/release/missing.dylib")
            .status()?
            .code();
        Ok(true)
    })?;

    assert_eq!(exit_code, None, "unexpected real exit code");
    assert_eq!(rm_result, RmResult::NotRemoved);
    Ok(())
}
