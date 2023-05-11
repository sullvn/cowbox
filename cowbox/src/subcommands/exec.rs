use cowbox::spawn;
use std::io::Result;
use std::process::ExitCode;

use crate::args::ExecArgs;
use crate::cache_dir::cache_dir_path;

/// Execute program within sandbox
///
/// Does not spawn a shell. The program
/// is directly ran as if it was called
/// via [`exec`][0].
///
/// [0]: https://en.wikipedia.org/wiki/Exec_(system_call)
///
pub fn exec(args: ExecArgs) -> Result<ExitCode> {
    let exit_code_i32 = spawn(cache_dir_path()?, args.program, args.program_args)?;

    // Smush spawned process return code
    // into smaller, standard [`ExitCode`][0].
    //
    // [`ExitCode::from_raw`][1] would be
    // useful here to avoid smushing on
    // Windows. Alas, it hasn't landed
    // on stable Rust yet.
    //
    // For now, default to [`ExitCode::FAILURE`][2]
    // if the spawned process exit code is too big,
    // as it can be on Windows.
    //
    // TODO: Move to using `ExitCode::from_raw`
    // when available.
    //
    // [0]: `std::process::ExitCode`
    // [1]: `std::process::ExitCode::from_raw`
    // [2]: `std::process::ExitCode::FAILURE`
    //
    let exit_code = TryInto::<u8>::try_into(exit_code_i32)
        .map(ExitCode::from)
        .unwrap_or(ExitCode::FAILURE);

    Ok(exit_code)
}
