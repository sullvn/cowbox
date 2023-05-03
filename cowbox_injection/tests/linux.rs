#![cfg(target_os = "linux")]

use cowbox_testing::{run_test_rm, RmResult};
use std::io::Result;
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
fn sandboxed_rm() -> Result<()> {
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, _| {
        Ok(Command::new("rm")
            .arg(file_path)
            .env_clear()
            .env(
                "LD_PRELOAD",
                concat!("../target/", env!("PROFILE"), "/libcowbox_injection.so"),
            )
            .status()?
            .success())
    })?;

    assert_eq!(rm_result, RmResult::NotRemoved);
    Ok(())
}

#[test]
fn missing_dylib_rm() -> Result<()> {
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, _| {
        Ok(Command::new("rm")
            .arg(file_path)
            .env_clear()
            .env(
                "LD_PRELOAD",
                concat!("../target/", env!("PROFILE"), "/missing.so"),
            )
            .status()?
            .success())
    })?;

    assert_eq!(rm_result, RmResult::Removed);
    Ok(())
}
