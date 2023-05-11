use clap::{Args, Parser};
use std::ffi::OsString;

///
/// Run any program without fear via a
/// copy-on-write file system sandbox
///
/// Examples:
///
/// - Rapid prototyping
/// - Example command from the internet
/// - Suggestion from AI
///
/// Example:
///
///     $ cowbox run rm -rf /
///
///     # Everything's still there :D
///     $ ls /
///
#[derive(Parser)]
#[command(verbatim_doc_comment)]
pub enum CliArgs {
    Exec(ExecArgs),
}

///
/// Execute a program in sandbox,
/// directly without shell
///
/// Arguments are passed in
/// directly to the program.
///
#[derive(Args)]
pub struct ExecArgs {
    #[arg(required = true)]
    pub program: OsString,

    #[arg(allow_hyphen_values = true)]
    pub program_args: Vec<OsString>,
}
