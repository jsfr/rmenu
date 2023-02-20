mod cli;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use std::{
    collections::{hash_map::RandomState, HashMap},
    fmt::Write,
    io::{self, prelude::*},
    path::PathBuf,
};

type HistoryItems = HashMap<String, i32, RandomState>;

fn parse_history_file(path: &PathBuf) -> Result<HistoryItems> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`.", path.to_string_lossy()))?;

    let history_items_result: Result<HistoryItems> = content
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.splitn(2, ':').collect();

            match split_line.as_slice() {
                [n, a] => match n.parse::<i32>() {
                    Ok(parsed_n) => Ok(((*a).to_string(), parsed_n)),
                    Err(_) => Err(anyhow!("could not parse `{}` as an integer.", n)),
                },
                _ => Err(anyhow!("could not split `{}`.", line)),
            }
        })
        .collect();

    history_items_result.context("could not parse history file.")
}

fn write_history_file(path: &PathBuf, history_items: &HistoryItems) -> Result<()> {
    let mut content = String::new();

    let mut sorted_items = history_items
        .iter()
        .map(|(a, n)| (*n, a))
        .collect::<Vec<(i32, &String)>>();

    sorted_items.sort_by_key(|(n, _)| *n);

    for (n, a) in sorted_items {
        writeln!(&mut content, "{n}:{a}")
            .with_context(|| format!("could not format values `{n}`, `{a}`."))?;
    }

    std::fs::write(path, content)
        .with_context(|| format!("failed to write content to `{}`.", path.to_string_lossy()))
}

fn sort(history_items: &HistoryItems, items: &mut Vec<String>) -> Result<()> {
    if items.is_empty() {
        *items = io::stdin()
            .lock()
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .context("failed to read items from stdin.")?;
    }

    items.sort_by(|a, b| {
        let x = match history_items.get(a) {
            Some(n) => (-*n, a.to_ascii_lowercase()),
            None => (0, a.to_ascii_lowercase()),
        };

        let y = match history_items.get(b) {
            Some(n) => (-*n, b.to_ascii_lowercase()),
            None => (0, b.to_ascii_lowercase()),
        };

        x.cmp(&y)
    });

    Ok(())
}

fn update(path: &PathBuf, entry: String) -> Result<()> {
    let mut history_items = parse_history_file(path)?;

    let n = match history_items.get(&entry) {
        Some(n) => *n + 1,
        None => 1,
    };
    history_items.insert(entry, n);

    write_history_file(path, &history_items)?;

    Ok(())
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    match args.cmd {
        Commands::Sort { mut items } => {
            let history_items = parse_history_file(&args.path)?;

            sort(&history_items, &mut items)?;

            // Output the sorted list
            println!("{}", items.join("\n"));

            Ok(())
        }
        Commands::Update { entry } => update(&args.path, entry),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn sort_sorts_alphabetically() -> Result<()> {
        let history_items = HistoryItems::new();
        let mut items = vec_of_strings!["Insync", "Firefox", "Spotify"];
        let sorted_items = vec_of_strings!["Firefox", "Insync", "Spotify"];

        sort(&history_items, &mut items)?;

        assert_eq!(items, sorted_items);

        Ok(())
    }

    #[test]
    fn sort_sorts_historical_items_first() -> Result<()> {
        let mut history_items = HistoryItems::new();

        history_items.insert("Insync".to_string(), 1);
        history_items.insert("Spotify".to_string(), 2);

        let mut items = vec_of_strings!["Insync", "Firefox", "Spotify"];
        let sorted_items = vec_of_strings!["Spotify", "Insync", "Firefox"];

        sort(&history_items, &mut items)?;

        assert_eq!(items, sorted_items);

        Ok(())
    }

    #[test]
    fn sort_sorts_case_insensitive() -> Result<()> {
        let history_items = HistoryItems::new();
        let mut items = vec_of_strings!["insync", "Firefox", "Spotify"];
        let sorted_items = vec_of_strings!["Firefox", "insync", "Spotify"];

        sort(&history_items, &mut items)?;

        assert_eq!(items, sorted_items);

        Ok(())
    }
}
