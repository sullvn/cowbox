mod args;

use args::Args;
use clap::Parser;

fn main() {
    match Args::parse() {
        Args::Run(args) => {
            println!("{}", args.command.len())
        }
    }
}
