#![cfg(target_os = "windows")]

use cowbox::DetourCreateProcessWithDllExA;
use std::io::Result;
use std::mem::zeroed;
use std::ptr;
use windows_sys::Win32::Foundation::{CloseHandle, FALSE, TRUE};
use windows_sys::Win32::System::Threading::CreateProcessA;
use windows_sys::Win32::System::Threading::{
    WaitForSingleObject, INFINITE, PROCESS_INFORMATION, STARTUPINFOA,
};

mod common;
use common::{run_test_rm, RmResult};

#[test]
fn normal_rm() -> Result<()> {
    run_test_rm(RmResult::Removed, |file_path, _| {
        let mut rm_cmd = format!(
            "powershell -Command \"Remove-Item {}\"",
            file_path.to_str().unwrap()
        );

        unsafe {
            let si: STARTUPINFOA = zeroed();
            let mut pi: PROCESS_INFORMATION = zeroed();
            let result = CreateProcessA(
                ptr::null(),
                rm_cmd.as_mut_ptr(),
                ptr::null(),
                ptr::null(),
                FALSE,
                0,
                ptr::null(),
                ptr::null(),
                &si,
                &mut pi,
            );

            WaitForSingleObject(pi.hProcess, INFINITE);
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);

            assert_eq!(result, TRUE, "rm command couldn't be executed");
        }

        Ok(())
    })
}

#[test]
fn sandboxed_rm() -> Result<()> {
    let test_arch = Arch::from_target();
    for rm_arch in Arch::options() {
        for dll_arch in Arch::options() {
            run_windows_test_rm(test_arch, rm_arch, dll_arch)?;
        }
    }

    Ok(())
}

enum Arch {
    X86,
    X86_64,
}

impl Arch {
    fn from_target() -> Self {
        if cfg!(target_arch = "x86") {
            Self::X86
        } else if cfg!(target_arch = "x86_64") {
            Self::X86_64
        } else {
            unimplemented!("Unsupported architecture")
        }
    }

    fn options() -> impl Iterator<Item = Self> {
        [Self::X86, Self::X86_64].into_iter()
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X86 => f.write_str("x86"),
            X86_64 => f.write_str("x86_64"),
        }
    }
}

fn run_windows_test_rm(test_arch: Arch, rm_arch: Arch, dll_arch: Arch) -> Result<()> {
    let rm_program = match rm_arch {
        X86 => "C:\\Windows\\SysWOW64\\WindowsPowerShell\\v1.0\\powershell.exe",
        X86_64 => "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
    };

    let dll_path = match dll_arch {
        X86 => "target\\release\\cowbox32.dll",
        X86_64 => "target\\release\\cowbox64.dll",
    };

    run_test_rm(RmResult::NotRemoved, |file_path, _| {
        let mut rm_cmd = format!(
            "{} -Command \"Remove-Item {}\"",
            rm_program,
            file_path.to_str().unwrap()
        );

        unsafe {
            let si: STARTUPINFOA = zeroed();
            let mut pi: PROCESS_INFORMATION = zeroed();
            let result = DetourCreateProcessWithDllExA(
                ptr::null(),
                rm_cmd.as_mut_ptr(),
                ptr::null(),
                ptr::null(),
                FALSE,
                0,
                ptr::null(),
                ptr::null(),
                &si,
                &mut pi,
                dll_path.as_ptr(),
                None,
            );

            WaitForSingleObject(pi.hProcess, INFINITE);
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);

            assert_eq!(
                result, TRUE,
                "rm command couldn't be executed: test={} rm={} dll={}",
                test_arch, rm_arch, dll_arch
            );
        }

        Ok(())
    })
}
