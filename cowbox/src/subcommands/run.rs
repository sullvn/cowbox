use cowbox::spawn;
use std::io::Result;
use std::process::ExitCode;

use crate::args;
use crate::cache_dir::cache_dir_path;

pub fn run(args: args::Run) -> Result<ExitCode> {
    spawn(cache_dir_path()?, args.program, args.program_args)
}
