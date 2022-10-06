use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path of the history file to sort by
    pub path: PathBuf,

    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
