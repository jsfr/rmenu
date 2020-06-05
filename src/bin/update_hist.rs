use exitfailure::ExitFailure;
use structopt::StructOpt;

use utils::parse_history_file;
use utils::write_history_file;

/// Updates a history file based on a new entry
#[derive(StructOpt)]
#[structopt(name = "update_hist", version = "0.1.0")]
struct Cli {
    /// The path of the history file to sort by
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    /// The new entry to add to the history file
    entry: String,
}

fn main() -> Result<(), ExitFailure> {
    let args: Cli = Cli::from_args();

    let mut history_items = parse_history_file(&args.path)?;
    let entry = history_items.get(&args.entry);
    let n = match entry {
        Some(n) => *n + 1,
        None => 1,
    };
    history_items.insert(args.entry, n);

    write_history_file(&args.path, history_items)?;

    Ok(())
}
