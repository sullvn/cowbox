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
fn sandboxed_sip_rm() -> Result<()> {
    run_test_rm(RmResult::Removed, |file_path, _| {
        let mut cmd = Command::new("rm");
        cmd.arg(file_path)
            .env_clear()
            .env("DYLD_INSERT_LIBRARIES", "target/release/libcowbox.dylib");

        assert!(cmd.status()?.success());

        Ok(())
    })
}

#[test]
fn sandboxed_rm() -> Result<()> {
    run_test_rm(RmResult::NotRemoved, |file_path, tmp_dir_path| {
        let rm_copy_path: PathBuf = [tmp_dir_path.as_ref(), "rm".as_ref()].iter().collect();
        fs::copy("/bin/rm", &rm_copy_path)?;

        let mut cmd = Command::new(rm_copy_path);
        cmd.arg(file_path)
            .env_clear()
            .env("DYLD_INSERT_LIBRARIES", "target/release/libcowbox.dylib");

        assert!(cmd.status()?.success());

        Ok(())
    })
}
