#![cfg(target_os = "windows")]

use cowbox_testing::{run_test_rm, Arch, RmResult, TempDir, TempPath};
use detours_sys::DetourCreateProcessWithDllExA;
use std::ffi::CString;
use std::fs;
use std::io::Result;
use std::mem::zeroed;
use std::path::Path;
use std::ptr;
use windows_sys::Win32::Foundation::{
    CloseHandle, BOOL, FALSE, STATUS_DLL_NOT_FOUND, STATUS_INVALID_IMAGE_FORMAT, TRUE,
};
use windows_sys::Win32::System::Threading::{
    CreateProcessA, GetExitCodeProcess, WaitForSingleObject, INFINITE, PROCESS_INFORMATION,
    STARTUPINFOA,
};

type ExitCode = u32;

#[test]
fn normal_rm() -> Result<()> {
    let (rm_result, exit_code) = run_windows_test_rm(|si, pi, file_path: &TempPath, _| {
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
    let dll_dir = TempDir::new_in(env!("CARGO_TARGET_TMPDIR"))?;
    fs::copy(
        concat!("..\\target\\i686-pc-windows-msvc\\", env!("PROFILE"), "\\cowbox_injection.dll"),
        dll_dir.path().join("cowbox_injection32.dll"),
    )?;
    fs::copy(
        concat!("..\\target\\x86_64-pc-windows-msvc\\", env!("PROFILE"), "\\cowbox_injection.dll"),
        dll_dir.path().join("cowbox_injection64.dll"),
    )?;

    let test_arch = Arch::from_target();
    for dll_arch in Arch::options() {
        for rm_arch in Arch::options() {
            sandboxed_rm_configuration(dll_dir.path(), &test_arch, &dll_arch, &rm_arch)?;
        }
    }

    Ok(())
}

#[test]
fn missing_dylib_rm() -> Result<()> {
    let (rm_result, exit_code) = run_windows_detour_rm(Path::new(".\\missing.dll"), "powershell")?;

    assert_eq!(
        rm_result,
        RmResult::NotRemoved,
        "file was unexpectedly removed"
    );
    assert_eq!(
        exit_code, STATUS_DLL_NOT_FOUND as u32,
        "exit code was unexpected"
    );

    Ok(())
}

fn sandboxed_rm_configuration(dll_dir: &Path, test_arch: &Arch, dll_arch: &Arch, rm_arch: &Arch) -> Result<()> {
    let dll_path = match dll_arch {
        Arch::X86 => dll_dir.join("cowbox_injection32.dll"),
        Arch::X86_64 => dll_dir.join("cowbox_injection64.dll"),
    };

    let rm_program = match rm_arch {
        Arch::X86 => "C:\\Windows\\SysWOW64\\WindowsPowerShell\\v1.0\\powershell.exe",
        Arch::X86_64 => "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
    };

    let (rm_result, exit_code) = run_windows_detour_rm(&dll_path, rm_program)?;

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

fn run_windows_detour_rm(dll_path: &Path, rm_program: &str) -> Result<(RmResult, ExitCode)> {
    run_windows_test_rm(|si, pi, file_path: &TempPath, _| {
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
                CString::new(dll_path.to_str().unwrap())
                    .unwrap()
                    .into_bytes_with_nul()
                    .as_ptr(),
                None,
            )
        }
    })
}

fn run_windows_test_rm<F>(create_process_fn: F) -> Result<(RmResult, ExitCode)>
where
    F: FnOnce(&STARTUPINFOA, &mut PROCESS_INFORMATION, &TempPath, &TempDir) -> BOOL,
{
    let mut exit_code: ExitCode = 0;

    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, tmp_dir| {
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
