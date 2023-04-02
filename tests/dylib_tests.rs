use std::fs::File;
use std::io::Result;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn sandboxed_rm() -> Result<()> {
    let path = NamedTempFile::new_in(env!("CARGO_TARGET_TMPDIR"))?.into_temp_path();
    assert!(File::open(&path).is_ok(), "test file wasn't created");

    let mut cmd = Command::new("rm");
    cmd.arg(&path).env_clear();

    if cfg!(target_os = "linux") {
        cmd.env("LD_PRELOAD", "target/release/libcowbox.so");
    } else if cfg!(target_os = "macos") {
        cmd.env("DYLD_INSERT_LIBRARIES", "target/release/libcowbox.dylib");
    }

    assert!(
        cmd.status()?.success(),
        "sandboxed rm has non-zero exit code"
    );
    assert!(File::open(&path).is_ok(), "test file was actually removed");

    Ok(())
}
