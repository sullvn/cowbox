use std::ffi::OsStr;
use std::fs::{self, create_dir_all, read_to_string};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(target_os = "linux")]
const INJECTION_LIB_FILE_NAME: &str = "libcowbox_injection.so";
#[cfg(target_os = "macos")]
const INJECTION_LIB_FILE_NAME: &str = "libcowbox_injection.dylib";
#[cfg(all(target_os = "windows", target_arch = "x86"))]
const INJECTION_LIB_FILE_NAME: &str = "cowbox_injection32.dll";
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const INJECTION_LIB_FILE_NAME: &str = "cowbox_injection64.dll";

#[cfg(target_os = "linux")]
const DYLIB_ENV_KEY: &str = "LD_PRELOAD";
#[cfg(target_os = "macos")]
const DYLIB_ENV_KEY: &str = "DYLD_INSERT_LIBRARIES";

const INJECTION_HASH: u128 = 0;
const INJECTION_HASH_FILE_NAME: &str = "hash";
const INJECTION_DIR_NAME: &str = "injection";

#[cfg(target_os = "linux")]
fn injection_binary() -> &'static [u8] {
    include_bytes!("../../target/release/libcowbox_injection.so")
}

#[cfg(target_os = "macos")]
fn injection_binary() -> &'static [u8] {
    include_bytes!("../../target/release/libcowbox_injection.dylib")
}

#[cfg(target_os = "windows")]
fn injection_binary() -> &'static [u8] {
    include_bytes!("../../target/release/cowbox_injection64.dll")
}

fn injection_lib_exists<P: AsRef<Path>>(lib_dir: P) -> Option<bool> {
    let lib_path: PathBuf = [lib_dir, INJECTION_DIR_NAME, INJECTION_LIB_FILE_NAME].iter().collect();
    let hash_path: PathBuf = [lib_dir, INJECTION_DIR_NAME, INJECTION_HASH_FILE_NAME].iter().collect();

    lib_path.try_exists().ok()?;

    let found_hash: u128 = read_to_string(hash_path).ok()?.parse().ok()?;
    let hash_matches = found_hash == INJECTION_HASH;

    Some(hash_matches)
}

fn injection_lib_create<P: AsRef<Path>>(lib_dir: P) -> Result<()> {
    let hash_str = format!("{:x}", INJECTION_HASH);

    let lib_dir: PathBuf = [lib_dir, INJECTION_DIR_NAME].iter().collect();
    let lib_path: PathBuf = [lib_dir, INJECTION_LIB_FILE_NAME].iter().collect();
    let hash_path: PathBuf = [lib_dir, INJECTION_HASH_FILE_NAME].iter().collect();

    create_dir_all(lib_dir)?;
    fs::write(lib_path, injection_binary())?;
    fs::write(hash_path, hash_str.as_bytes())?;

    Ok(())
}

fn injection_lib_update<P: AsRef<Path>>(lib_dir: P) -> Result<()> {
    if let Some(true) = injection_lib_exists(lib_dir) {
        return Ok(());
    }

    injection_lib_create(lib_dir)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn spawn_injected_process<P, S, T, A>(lib_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    let lib_path: PathBuf = [lib_dir, INJECTION_DIR_NAME, INJECTION_LIB_FILE_NAME].iter().collect();
    let exit_code = Command::new(program)
        .args(args)
        .env(DYLIB_ENV_KEY, lib_path)
        .status()?
        .code();
    Ok(exit_code)
}

#[cfg(target_os = "windows")]
fn spawn_injected_process<P, S, T, A>(lib_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    unsafe {
        let si: STARTUPINFOA = zeroed();
        let mut pi: PROCESS_INFORMATION = zeroed();
        let result = DetourCreateProcessWithDllExA(
            ptr::null(), // TODO: use program
            rm_cmd.into_bytes_with_nul().as_mut_ptr(), // TODO: use args
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
        );
        assert_eq!(result, TRUE, "process couldn't be created executed");

        WaitForSingleObject(pi.hProcess, INFINITE);

        let result = GetExitCodeProcess(pi.hProcess, &mut exit_code);
        assert_eq!(result, TRUE, "exit code couldn't be retrieved");

        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
    }
}

pub fn spawn<P, S, T, A>(lib_dir: P, program: S, args: A) -> Result<i32>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    injection_lib_update(lib_dir)?;
    spawn_injected_process(lib_dir, program, args)?;
}
