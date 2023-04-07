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
    run_test_rm(RmResult::NotRemoved, |file_path, _| {
        let mut rm_cmd = format!(
            "powershell -Command \"Remove-Item {}\"",
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
                "target/release/cowbox.dll".as_ptr(),
                None,
            );

            WaitForSingleObject(pi.hProcess, INFINITE);
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);

            assert_eq!(result, TRUE, "rm command couldn't be executed");
        }

        Ok(())
    })
}
