mod cli;
mod ui;
mod ui_args;
mod ui_data;
mod ui_delegate;

use crate::{cli::Cli, ui::run_selector, ui_args::Args};
use anyhow::{Context, Result};
use clap::Parser;
use druid::im::Vector;
use std::io::{prelude::*, stdin};

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    let items = match cli.items {
        Some(ref i) if !i.is_empty() => Ok(i.clone()),
        _ => stdin()
            .lock()
            .lines()
            .collect::<Result<Vector<String>, _>>(),
    }
    .context("failed to read items from stdin.")?;

    let ui_args: Args = Args::from(&cli, items);

    run_selector(ui_args).context("Failed to start rmenu")?;

    Ok(())
}
