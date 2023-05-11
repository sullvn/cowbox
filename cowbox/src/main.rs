mod args;
mod cache_dir;
mod subcommands;

use clap::Parser;
use std::io::Result;
use std::process::ExitCode;

use args::CliArgs;
use subcommands::exec;

fn main() -> Result<ExitCode> {
    match CliArgs::parse() {
        CliArgs::Exec(args) => exec(args),
    }
}
