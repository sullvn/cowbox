pub mod constants;
mod injection_binaries;
mod injection_binary;

use injection_binaries::InjectionBinaries;
use injection_binary::InjectionBinary;

#[cfg(target_os = "linux")]
pub const INJECTION_BINARIES: InjectionBinaries = InjectionBinaries::new(&[InjectionBinary::new(
    "libcowbox_injection.so",
    include_bytes!(concat!(
        "../../target/",
        env!("PROFILE"),
        "/libcowbox_injection.so",
    )),
)]);

#[cfg(target_os = "macos")]
pub const INJECTION_BINARIES: InjectionBinaries = InjectionBinaries::new(&[InjectionBinary::new(
    "libcowbox_injection.dylib",
    include_bytes!(concat!(
        "../../target/",
        env!("PROFILE"),
        "/libcowbox_injection.dylib",
    )),
)]);

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub const INJECTION_BINARIES: InjectionBinaries = InjectionBinaries::new(&[
    InjectionBinary::new(
        "cowbox_injection32.dll",
        include_bytes!(concat!(
            "../../target/i686-pc-windows-msvc/",
            env!("PROFILE"),
            "/cowbox_injection.dll",
        )),
    ),
    InjectionBinary::new(
        "cowbox_injection64.dll",
        include_bytes!(concat!(
            "../../target/x86_64-pc-windows-msvc/",
            env!("PROFILE"),
            "/cowbox_injection.dll",
        )),
    ),
]);

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const INJECTION_BINARIES: InjectionBinaries = InjectionBinaries::new(&[
    InjectionBinary::new(
        "cowbox_injection64.dll",
        include_bytes!(concat!(
            "../../target/x86_64-pc-windows-msvc/",
            env!("PROFILE"),
            "/cowbox_injection.dll",
        )),
    ),
    InjectionBinary::new(
        "cowbox_injection32.dll",
        include_bytes!(concat!(
            "../../target/i686-pc-windows-msvc/",
            env!("PROFILE"),
            "/cowbox_injection.dll",
        )),
    ),
]);
