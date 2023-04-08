#![cfg(target_os = "windows")]

use cowbox::DetourCreateProcessWithDllExA;
use std::ffi::CString;
use std::io::Result;
use std::mem::zeroed;
use std::ptr;
use tempfile::{TempDir, TempPath};
use windows_sys::Win32::Foundation::{CloseHandle, BOOL, FALSE, STATUS_INVALID_IMAGE_FORMAT, TRUE};
use windows_sys::Win32::System::Threading::{
    CreateProcessA, GetExitCodeProcess, WaitForSingleObject, INFINITE, PROCESS_INFORMATION,
    STARTUPINFOA,
};

mod common;
use common::{run_test_rm, Arch, RmResult};

type ExitCode = u32;

#[test]
fn normal_rm() -> Result<()> {
    let (rm_result, exit_code) = run_windows_test_rm(|si, pi, file_path, _| {
        let rm_cmd = CString::new(format!(
            "powershell -Command \"Remove-Item {}\"",
            file_path.to_str().unwrap()
        ))
        .unwrap();

        unsafe {
            CreateProcessA(
                ptr::null(),
                rm_cmd.into_bytes_with_nul().as_mut_ptr(),
                ptr::null(),
                ptr::null(),
                FALSE,
                0,
                ptr::null(),
                ptr::null(),
                si,
                pi,
            )
        }
    })?;

    assert_eq!(rm_result, RmResult::Removed);
    assert_eq!(exit_code, 0, "non-zero exit code");
    Ok(())
}

#[test]
fn sandboxed_rm() -> Result<()> {
    let test_arch = Arch::from_target();
    for dll_arch in Arch::options() {
        for rm_arch in Arch::options() {
            sandboxed_rm_configuration(&test_arch, &dll_arch, &rm_arch)?;
        }
    }

    Ok(())
}

fn sandboxed_rm_configuration(test_arch: &Arch, dll_arch: &Arch, rm_arch: &Arch) -> Result<()> {
    let dll_path = match dll_arch {
        Arch::X86 => "target\\release\\cowbox32.dll",
        Arch::X86_64 => "target\\release\\cowbox64.dll",
    };

    let rm_program = match rm_arch {
        Arch::X86 => "C:\\Windows\\SysWOW64\\WindowsPowerShell\\v1.0\\powershell.exe",
        Arch::X86_64 => "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
    };

    let (rm_result, exit_code) = run_windows_test_rm(|si, pi, file_path, _| {
        let rm_cmd = CString::new(format!(
            "{} -Command \"Remove-Item {}\"",
            rm_program,
            file_path.to_str().unwrap()
        ))
        .unwrap();

        unsafe {
            DetourCreateProcessWithDllExA(
                ptr::null(),
                rm_cmd.into_bytes_with_nul().as_mut_ptr(),
                ptr::null(),
                ptr::null(),
                FALSE,
                0,
                ptr::null(),
                ptr::null(),
                si,
                pi,
                CString::new(dll_path)
                    .unwrap()
                    .into_bytes_with_nul()
                    .as_ptr(),
                None,
            )
        }
    })?;

    let expected_exit_code: u32 = match (&test_arch, &dll_arch, &rm_arch) {
        (ta, da, _) if ta == da => 0,
        (Arch::X86_64, Arch::X86, Arch::X86) => 0,
        _ => STATUS_INVALID_IMAGE_FORMAT as u32,
    };

    assert_eq!(
        rm_result,
        RmResult::NotRemoved,
        "file was unexpectedly removed"
    );
    assert_eq!(exit_code, expected_exit_code, "exit code was unexpected");

    Ok(())
}

fn run_windows_test_rm<F>(create_process_fn: F) -> Result<(RmResult, ExitCode)>
where
    F: FnOnce(&STARTUPINFOA, &mut PROCESS_INFORMATION, &TempPath, &TempDir) -> BOOL,
{
    let mut exit_code: ExitCode = 0;

    let rm_result = run_test_rm(|file_path, tmp_dir| {
        unsafe {
            let si: STARTUPINFOA = zeroed();
            let mut pi: PROCESS_INFORMATION = zeroed();
            let result = create_process_fn(&si, &mut pi, file_path, tmp_dir);
            assert_eq!(result, TRUE, "process couldn't be created executed");

            WaitForSingleObject(pi.hProcess, INFINITE);

            let result = GetExitCodeProcess(pi.hProcess, &mut exit_code);
            assert_eq!(result, TRUE, "exit code couldn't be retrieved");

            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);
        }

        Ok(true)
    })?;

    Ok((rm_result, exit_code))
}
