#[cfg(windows)]
mod arch;
mod test_rm;

#[cfg(windows)]
pub use arch::Arch;
pub use test_rm::{run_test_rm, RmResult};
