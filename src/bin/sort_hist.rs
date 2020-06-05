use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

use utils::parse_history_file;

/// Sorts a list of items based on a history file.
#[derive(StructOpt)]
#[structopt(name = "sort_hist", version = "0.1.0")]
struct Cli {
    /// The path of the history file to sort by
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    /// The items to sort, default to stdin
    items: Vec<String>,
}

fn main() -> Result<(), ExitFailure> {
    let args: Cli = Cli::from_args();

    let history_items = parse_history_file(&args.path)?;

    let mut items = match args.items.len() {
        0 => io::stdin()
            .lock()
            .lines()
            .collect::<Result<Vec<String>, _>>(),
        _ => Ok(args.items),
    }
    .context("failed to read items from stdin.")?;

    items.sort_by_key(|item| match history_items.get(item) {
        Some(n) => (-*n, item.to_owned()),
        None => (0, item.to_owned()),
    });

    // Output the sorted list
    println!("{}", items.join("\n"));

    Ok(())
}
