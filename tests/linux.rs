#![cfg(target_os = "linux")]

use std::io::Result;
use std::process::Command;

mod common;
use common::{run_test_rm, RmResult};

#[test]
fn normal_rm() -> Result<()> {
    let rm_result = run_test_rm(|file_path, _| {
        Ok(Command::new("rm")
            .arg(file_path)
            .env_clear()
            .status()?
            .success())
    });

    assert_eq!(rm_result, RmResult::Removed);
    Ok(())
}

#[test]
fn sandboxed_rm() -> Result<()> {
    let rm_result = run_test_rm(|file_path, _| {
        Ok(Command::new("rm")
            .arg(file_path)
            .env_clear()
            .env("LD_PRELOAD", "target/release/libcowbox.so")
            .status()?
            .success())
    });

    assert_eq!(rm_result, RmResult::NotRemoved);
    Ok(())
}
