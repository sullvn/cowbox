mod unix;
mod windows;

#[cfg(unix)]
pub use unix::spawn_process;

#[cfg(windows)]
pub use windows::spawn_process;
