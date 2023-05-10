mod args;
mod cache_dir;
mod subcommands;

use clap::Parser;
use std::io::Result;
use std::process::ExitCode;

use args::Args;
use subcommands::exec;

fn main() -> Result<ExitCode> {
    match Args::parse() {
        Args::Exec(args) => exec(args),
    }
}
