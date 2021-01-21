mod cli;
mod ui;
mod ui_args;
mod ui_data;
mod ui_delegate;

use crate::{cli::Cli, ui::run_selector, ui_args::Args};
use druid::im::Vector;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{prelude::*, stdin};
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let cli: Cli = Cli::from_args();

    let items = match cli.items {
        Some(ref i) if i.len() > 0 => Ok(i.clone()),
        _ => stdin()
            .lock()
            .lines()
            .collect::<Result<Vector<String>, _>>(),
    }
    .context("failed to read items from stdin.")?;

    let ui_args: Args = Args::from(cli, items);

    run_selector(ui_args).context("Failed to start rmenu")?;

    Ok(())
}
