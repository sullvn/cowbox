use cowbox::spawn;
use std::io::{Error, ErrorKind, Result};
use std::process::ExitCode;

use crate::args;
use crate::cache_dir::cache_dir_path;

/// Execute program within sandbox
///
/// Does not spawn a shell. The program
/// is directly ran as if it was called
/// via [`exec`][0].
///
/// [0]: https://en.wikipedia.org/wiki/Exec_(system_call)
///
pub fn exec(args: args::Exec) -> Result<ExitCode> {
    let exit_code_i32 = spawn(cache_dir_path()?, args.program, args.program_args)?;

    // Smush spawned process return code
    // into [`ExitCode`][0].
    //
    // [`ExitCode::from_raw`][1] would be
    // useful here to avoid smushing on
    // Windows. Alas, it hasn't landed
    // on stable Rust yet.
    //
    // In any case, the returned value
    // from [`cowbox::spawn`] would have
    // to be a platform-agnostic wrapper.
    //
    // [0]: `std::process::ExitCode`
    // [1]: `std::process::ExitCode::from_raw`
    //
    let exit_code_u8: u8 = exit_code_i32
        .try_into()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;
    let exit_code = ExitCode::from(exit_code_u8);

    Ok(exit_code)
}
