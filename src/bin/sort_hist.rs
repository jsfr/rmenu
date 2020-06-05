use exitfailure::ExitFailure;
use structopt::StructOpt;

use utils::parse_history_file;

/// Sorts a list of items based on a history file.
#[derive(StructOpt)]
#[structopt(name = "sort_hist", version = "0.1.0")]
struct Cli {
    /// The path of the history file to sort by
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    /// The items to sort
    items: Vec<String>,
}

fn main() -> Result<(), ExitFailure> {
    let mut args: Cli = Cli::from_args();

    let history_items = parse_history_file(&args.path)?;

    args.items
        .sort_by_key(|item| match history_items.get(item.as_str()) {
            Some(n) => -*n,
            None => 0,
        });

    // Output the sorted list
    println!("{}", args.items.join("\n"));

    Ok(())
}
