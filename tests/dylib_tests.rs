use std::fs::{self, File};
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;
use std::ptr;
use tempfile::{NamedTempFile, TempDir};

#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::{CloseHandle, FALSE};
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Threading::{
    WaitForSingleObject, INFINITE, PROCESS_INFORMATION, STARTUPINFOA,
};

#[cfg(target_os = "windows")]
use cowbox::DetourCreateProcessWithDllEx;

#[test]
fn sandboxed_rm() -> Result<()> {
    let cargo_tmp_dir_path = env!("CARGO_TARGET_TMPDIR");
    let tmp_dir_path = TempDir::new_in(cargo_tmp_dir_path)?;
    let rm_file_path = NamedTempFile::new_in(cargo_tmp_dir_path)?.into_temp_path();

    assert!(
        File::open(&rm_file_path).is_ok(),
        "test file wasn't created"
    );

    if cfg!(unix) {
        let rm_program = if cfg!(target_os = "linux") {
            "rm".into()
        } else if cfg!(target_os = "macos") {
            let rm_copy_path: PathBuf = [tmp_dir_path.as_ref(), "rm".as_ref()].iter().collect();
            fs::copy("/bin/rm", &rm_copy_path)?;

            rm_copy_path
        } else {
            unimplemented!("rm program")
        };

        let mut cmd = Command::new(rm_program);
        cmd.arg(&rm_file_path).env_clear();

        if cfg!(target_os = "linux") {
            cmd.env("LD_PRELOAD", "target/release/libcowbox.so");
        } else if cfg!(target_os = "macos") {
            cmd.env("DYLD_INSERT_LIBRARIES", "target/release/libcowbox.dylib");
        }

        assert!(
            cmd.status()?.success(),
            "sandboxed rm has non-zero exit code"
        );
    } else {
        let si: STARTUPINFOA;
        let mut pi: PROCESS_INFORMATION;
        let result = DetourCreateProcessWithDllEx(
            ptr::null(),
            format!(
                "pwsh -command \". 'rm {}'\"",
                &rm_file_path.to_str().unwrap()
            ),
            ptr::null(),
            ptr::null(),
            FALSE,
            0,
            ptr::null(),
            ptr::null(),
            &si,
            &mut pi,
            "target/release/cowbox.dll",
            None,
        );

        WaitForSingleObject(pi.hProcess, INFINITE);
        CloseHandle(pi.hProcess);
        CloseHandle(pi.pThread);
    }

    assert!(
        File::open(&rm_file_path).is_ok(),
        "test file was actually removed"
    );

    Ok(())
}
