use std::ffi::OsStr;
use std::path::Path;

#[cfg(target_os = "linux")]
pub fn injection_binary() -> &'static [u8] {
    include_bytes!("../../target/release/libcowbox_injection.so")
}

#[cfg(target_os = "macos")]
pub fn injection_binary() -> &'static [u8] {
    include_bytes!("../../target/release/libcowbox_injection.dylib")
}

#[cfg(target_os = "windows")]
pub fn injection_binary() -> &'static [u8] {
    include_bytes!("../../target/release/cowbox_injection64.dll")
}

fn cowbox_spawn<P, S, T, A>(lib_dir: P, program: S, args: A) -> i32
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    A: IntoIterator<Item = T>,
{
    // 1. Recreate injection libs if needed
    //    a. (Possible? Needs testing) Try to run with dylib, recreate if error
    //    b. Check if dylib exists
    //    c. Check if dylib hash matches
    //    d. If so, create directory path
    //    e. Copy dylib and hash to fs
    //    f. Proceed to CLI program
    // 2. Spawn CLI program
    //    - Linux
    //    - macOS
    //    - Windows
}

#[cfg(test)]
mod tests {
    use super::injection_binary;

    #[test]
    fn it_works() {
        assert!(injection_binary().len() > 0);
    }
}
