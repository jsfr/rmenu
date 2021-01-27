use clap::Clap;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::{
    collections::{hash_map::RandomState, HashMap},
    fmt::Write,
    io::{self, prelude::*},
    path::PathBuf,
};

// TODO generalize from RandomState to generic
type HistoryItems = HashMap<String, i32, RandomState>;

/// Sorts a list of items based on a history file.
#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Jens Fredskov <jsfr@users.noreply.github.com>"
)]
struct Cli {
    /// The path of the history file to sort by
    #[clap(parse(from_os_str))]
    path: PathBuf,
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Clap)]
enum Command {
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

fn parse_history_file(path: &PathBuf) -> Result<HistoryItems, ExitFailure> {
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("could not read file `{}`.", path.to_string_lossy()))?;

    let history_items_result: Result<HistoryItems, _> = content
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.splitn(2, ':').collect();

            match split_line.as_slice() {
                [n, a] => match n.parse::<i32>() {
                    Ok(parsed_n) => Ok((a.to_string(), parsed_n)),
                    _ => Err(failure::err_msg(format!(
                        "could not parse `{}` as an integer.",
                        n
                    ))),
                },
                _ => Err(failure::err_msg(format!("could not split `{}`.", line))),
            }
        })
        .collect();

    Ok(history_items_result.context(
        "could not parse history file.
                 each line should have the form `[number]:[item]`.
                 example: `1:Firefox`.",
    )?)
}

fn write_history_file(path: &PathBuf, history_items: HistoryItems) -> Result<(), ExitFailure> {
    let mut content = String::new();

    for (a, n) in history_items.iter() {
        writeln!(&mut content, "{}:{}", n, a)
            .with_context(|_| format!("could not format values `{}`, `{}`.", n, a))?;
    }

    Ok(std::fs::write(path, content)
        .with_context(|_| format!("failed to write content to `{}`.", path.to_string_lossy()))?)
}

fn sort(path: &PathBuf, items: &mut Vec<String>) -> Result<(), ExitFailure> {
    let history_items = parse_history_file(path)?;

    if items.is_empty() {
        *items = io::stdin()
            .lock()
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .context("failed to read items from stdin.")?
    }

    items.sort_by(|a, b| {
        let x = match history_items.get(a) {
            Some(n) => (*n, a),
            None => (0, a),
        };

        let y = match history_items.get(b) {
            Some(n) => (*n, b),
            None => (0, b),
        };

        y.cmp(&x)
    });

    // Output the sorted list
    println!("{}", items.join("\n"));

    Ok(())
}

fn update(path: &PathBuf, entry: String) -> Result<(), ExitFailure> {
    let mut history_items = parse_history_file(path)?;
    let n = match history_items.get(&entry) {
        Some(n) => *n + 1,
        None => 1,
    };
    history_items.insert(entry, n);

    write_history_file(path, history_items)?;

    Ok(())
}

fn main() -> Result<(), ExitFailure> {
    let args: Cli = Cli::parse();

    match args.cmd {
        Command::Sort { mut items } => sort(&args.path, &mut items),
        Command::Update { entry } => update(&args.path, entry),
    }
}
