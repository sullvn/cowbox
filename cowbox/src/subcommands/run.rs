use cowbox::spawn;
use std::io::{Error, ErrorKind, Result};
use std::process::ExitCode;

use crate::args;
use crate::cache_dir::cache_dir_path;

pub fn run(args: args::Run) -> Result<ExitCode> {
    let exit_code_i32 = spawn(cache_dir_path()?, args.program, args.program_args)?;
    let exit_code_u8: u8 = exit_code_i32
        .try_into()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;
    let exit_code = ExitCode::from(exit_code_u8);

    Ok(exit_code)
}
