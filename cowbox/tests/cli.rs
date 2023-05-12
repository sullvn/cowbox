use cowbox_testing::{run_test_rm, RmResult, TempDir};
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
    let rm_result = run_test_rm(TMP_DIR, |file_path, tmp_dir| {
        let (program, args) = rm_program_and_args(file_path);
        let mut cmd = Command::new(program);
        cmd.args(args);
        sanitize_environment(&mut cmd, tmp_dir.path());

        Ok(cmd.status()?.success())
    })?;
    assert_eq!(rm_result, RmResult::Removed);

    //
    // Sandboxed
    //
    let rm_result = run_test_rm(TMP_DIR, |file_path, tmp_dir| {
        let (program, args) = rm_program_and_args(file_path);
        let mut cmd = Command::new(TEST_BINARY);
        cmd.arg("exec").arg(program).args(args);
        sanitize_environment(&mut cmd, tmp_dir.path());

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
        cmd.arg("exec").arg(program).args(args);
        sanitize_environment(&mut cmd, tmp_dir.path());

        let success = cmd.status()?.success();
        assert!(success, "command failed");

        let cache_dir = cache_dir_path(tmp_dir.path());
        assert!(cache_dir.is_dir(), "cache directory was not created");

        Ok(success)
    })?;

    Ok(())
}

#[test]
#[cfg(unix)]
fn xdg_cache_dir_precedence() -> Result<()> {
    //
    // $HOME is correctly used if $XDG_CACHE_HOME
    // is unavailable
    //
    run_test_rm(TMP_DIR, |file_path, tmp_dir| {
        let (program, args) = rm_program_and_args(file_path);
        let mut cmd = Command::new(TEST_BINARY);
        cmd.arg("exec").arg(program).args(args);
        sanitize_environment(&mut cmd, tmp_dir.path());

        cmd.env_remove("XDG_CACHE_HOME");
        cmd.env("HOME", tmp_dir.path());

        let success = cmd.status()?.success();
        assert!(success, "command failed");

        let cache_dir = home_cache_dir_path(tmp_dir.path());
        assert!(cache_dir.is_dir(), "cache directory was not created");

        Ok(success)
    })?;

    //
    // $HOME is not used if $XDG_CACHE_HOME
    // is available
    //
    run_test_rm(TMP_DIR, |file_path, tmp_dir| {
        let (program, args) = rm_program_and_args(file_path);
        let mut cmd = Command::new(TEST_BINARY);
        cmd.arg("exec").arg(program).args(args);
        sanitize_environment(&mut cmd, tmp_dir.path());

        let wrong_tmp_dir = TempDir::new_in(TMP_DIR)?;
        cmd.env("XDG_CACHE_HOME", tmp_dir.path());
        cmd.env("HOME", wrong_tmp_dir.path());

        let success = cmd.status()?.success();
        assert!(success, "command failed");

        let cache_dir_via_xdg = xdg_cache_dir_path(tmp_dir.path());
        assert!(
            cache_dir_via_xdg.is_dir(),
            "cache directory was not created"
        );

        let cache_dir_via_home = home_cache_dir_path(wrong_tmp_dir.path());
        assert!(
            !cache_dir_via_home.is_dir(),
            "HOME was incorrectly used to make cache dir"
        );

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

fn sanitize_environment(cmd: &mut Command, tmp_dir: &Path) {
    cmd.env_clear().env(HOME_ENV_KEY, tmp_dir).env(
        "PATH",
        var_os("PATH").expect("`PATH` environment variable is unset"),
    );

    // `SystemRoot` environment variable is required
    // for Windows PowerShell to work. Not sure why.
    if cfg!(windows) {
        cmd.env(
            "SystemRoot",
            var_os("SystemRoot").expect("`SystemRoot` environment variable is unset"),
        );
    }
}

#[cfg(unix)]
fn cache_dir_path(xdg_cache_home: &Path) -> PathBuf {
    xdg_cache_dir_path(xdg_cache_home)
}

#[cfg(windows)]
fn cache_dir_path(local_app_data: &Path) -> PathBuf {
    [local_app_data, PROJECT_NAME.as_ref(), "cache".as_ref()]
        .iter()
        .collect()
}

#[cfg(unix)]
fn xdg_cache_dir_path(xdg_cache_home: &Path) -> PathBuf {
    xdg_cache_home.join(PROJECT_NAME)
}

#[cfg(unix)]
fn home_cache_dir_path(home: &Path) -> PathBuf {
    [home, ".cache".as_ref(), PROJECT_NAME.as_ref()]
        .iter()
        .collect()
}
