mod injection_binary;
mod injection_binaries;
mod constants;

use injection_binaries::InjectionBinaries;
use injection_binary::InjectionBinary;

pub use constants::INJECTION_ENV_KEY;

#[cfg(target_os = "linux")]
pub const INJECTION_BINARIES: InjectionBinaries = InjectionBinaries::new(&[
    InjectionBinary::new(
        "libcowbox_injection.so",
        include_bytes!("../../target/release/libcowbox_injection.so"),
    ),
]);

#[cfg(target_os = "macos")]
pub const INJECTION_BINARIES: InjectionBinaries = InjectionBinaries::new(&[
    InjectionBinary::new(
        "libcowbox_injection.dylib",
        include_bytes!("../../target/release/libcowbox_injection.dylib"),
    ),
]);

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub const INJECTION_BINARIES: InjectionBinaries = InjectionBinaries::new(&[
    InjectionBinary::new(
        "cowbox_injection32.dll",
        include_bytes!("../../target/release/cowbox_injection32.dll"),
    ),
    InjectionBinary::new(
        "cowbox_injection64.dll",
        include_bytes!("../../target/release/cowbox_injection64.dll"),
    ),
]);

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const INJECTION_BINARIES: InjectionBinaries = InjectionBinaries::new(&[
    InjectionBinary::new(
        "cowbox_injection64.dll",
        include_bytes!("../../target/release/cowbox_injection64.dll"),
    ),
    InjectionBinary::new(
        "cowbox_injection32.dll",
        include_bytes!("../../target/release/cowbox_injection32.dll"),
    ),
]);