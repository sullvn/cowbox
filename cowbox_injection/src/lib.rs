#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
mod detours;

#[cfg(target_os = "windows")]
pub use detours::DetourCreateProcessWithDllExA;