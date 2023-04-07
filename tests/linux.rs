#![cfg(target_os = "macos")]

use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

mod common;
use common::{run_test_rm, RmResult};

#[test]
fn normal_rm() -> Result<()> {
    run_test_rm(RmResult::Removed, |file_path, _| {
        let mut cmd = Command::new("rm");
        cmd.arg(file_path).env_clear();

        assert!(cmd.status()?.success());

        Ok(())
    })
}

#[test]
fn sandboxed_rm() -> Result<()> {
    run_test_rm(RmResult::NotRemoved, |file_path, _| {
        let mut cmd = Command::new("rm");
        cmd.arg(file_path)
            .env_clear()
            .env("LD_PRELOAD", "target/release/libcowbox.so");

        assert!(cmd.status()?.success());

        Ok(())
    })
}
