use clap::Parser;
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
pub enum Args {
    Run(Run),
}

///
/// Run a command with sandboxed
/// file system access
///
#[derive(Parser)]
pub struct Run {
    #[arg(required = true)]
    pub command: Vec<OsString>,
}
