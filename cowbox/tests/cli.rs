use cowbox_testing::{run_test_rm, RmResult};
use std::env::var_os;
use std::ffi::OsString;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const TEST_BINARY: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));
const TMP_DIR: &str = env!("CARGO_TARGET_TMPDIR");

#[cfg(unix)]
const HOME_ENV_KEY: &str = "XDG_CACHE_HOME";

#[cfg(windows)]
const HOME_ENV_KEY: &str = "LOCALAPPDATA";

#[test]
fn exec_sandboxed_rm() -> Result<()> {
    //
    // Control -- Not sandboxed
    //
    let rm_result = run_test_rm(TMP_DIR, |file_path, _| {
        let (program, args) = rm_program_and_args(file_path);
        let mut cmd = Command::new(program);
        cmd.args(args).env_clear();

        Ok(cmd.status()?.success())
    })?;
    assert_eq!(rm_result, RmResult::Removed);

    //
    // Sandboxed
    //
    let rm_result = run_test_rm(TMP_DIR, |file_path, tmp_dir| {
        let (program, args) = rm_program_and_args(file_path);
        let mut cmd = Command::new(TEST_BINARY);
        cmd.arg("exec")
            .arg(program)
            .args(args)
            .env_clear()
            .env(HOME_ENV_KEY, tmp_dir.path())
            .env(
                "PATH",
                var_os("PATH").expect("$PATH environment variable is unset"),
            );

        Ok(cmd.status()?.success())
    })?;
    assert_eq!(rm_result, RmResult::NotRemoved);

    Ok(())
}

#[test]
fn cache_dir_created() -> Result<()> {
    run_test_rm(TMP_DIR, |file_path, tmp_dir| {
        let (program, args) = rm_program_and_args(file_path);
        let mut cmd = Command::new(TEST_BINARY);
        cmd.arg("exec")
            .arg(program)
            .args(args)
            .env_clear()
            .env(HOME_ENV_KEY, tmp_dir.path())
            .env(
                "PATH",
                var_os("PATH").expect("$PATH environment variable is unset"),
            );
        let success = cmd.status()?.success();
        assert!(success, "command failed");

        let cache_dir = cache_dir_path(tmp_dir.path());
        dbg!(&cache_dir);
        assert!(cache_dir.is_dir(), "cache directory was not created");

        Ok(success)
    })?;

    Ok(())
}

#[cfg(unix)]
fn rm_program_and_args(file_path: &Path) -> (OsString, Vec<OsString>) {
    ("rm".into(), vec![file_path.into()])
}

#[cfg(windows)]
fn rm_program_and_args(file_path: &Path) -> (OsString, Vec<OsString>) {
    let mut powershell_rm = OsString::from("Remove-Item ");
    powershell_rm.push(file_path);

    ("powershell".into(), vec!["-Command".into(), powershell_rm])
}

#[cfg(unix)]
fn cache_dir_path(home_path: &Path) -> PathBuf {
    [home_path, ".cache".as_ref(), PROJECT_NAME.as_ref()]
        .iter()
        .collect()
}

#[cfg(windows)]
fn cache_dir_path(home_path: &Path) -> PathBuf {
    [home_path, PROJECT_NAME.as_ref(), "cache".as_ref()]
        .iter()
        .collect()
}
