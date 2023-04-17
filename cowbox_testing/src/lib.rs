mod arch;
mod test_rm;

pub use arch::Arch;
pub use test_rm::{run_test_rm, RmResult};
pub use tempfile::{TempDir, TempPath};
