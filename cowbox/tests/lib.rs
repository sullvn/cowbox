use cowbox::spawn;
use cowbox_testing::{run_test_rm, RmResult};
use std::io::Result;

#[test]
fn sandboxed_sip_rm() -> Result<()> {
    let rm_result = run_test_rm(env!("CARGO_TARGET_TMPDIR"), |file_path, tmp_dir| {
        dbg!(tmp_dir);
        dbg!("rm");
        dbg!(file_path);
        let exit_code = spawn(tmp_dir, "rm", [file_path])?;
        Ok(exit_code == 0)
    })?;

    assert_eq!(rm_result, RmResult::NotRemoved);
    Ok(())
}