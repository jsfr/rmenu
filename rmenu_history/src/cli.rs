use clap::Parser;
use std::path::PathBuf;

/// Sorts a list of items based on a history file.
#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Jens Fredskov <jsfr@users.noreply.github.com>"
)]
pub struct Cli {
    /// The path of the history file to sort by
    #[clap(parse(from_os_str))]
    pub path: PathBuf,
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Parser)]
pub enum Command {
    /// Sort a list of items according to the history file
    Sort {
        /// The items to sort, default to stdin
        items: Vec<String>,
    },
    /// Update the history file with a new entry
    Update {
        /// The new entry to add to the history file
        entry: String,
    },
}
