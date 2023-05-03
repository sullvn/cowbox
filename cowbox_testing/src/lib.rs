mod arch;
mod test_rm;

pub use arch::Arch;
pub use tempfile::{TempDir, TempPath};
pub use test_rm::{run_test_rm, RmResult};
