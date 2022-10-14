mod cli;
mod filter;
mod item_filter;
mod ui;
mod ui_data;
mod ui_delegate;

use crate::{cli::Cli, ui::run_selector};
use anyhow::{Context, Result};
use clap::Parser;
use druid::{im::Vector, ArcStr};
use filter::Filter;
use std::io::{prelude::*, stdin};

#[derive(Clone, druid::Data)]
pub struct Item {
    pub key: ArcStr,
    pub value: ArcStr,
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    let filter = Filter::new(&cli.json_filter);

    let items: Vector<Item> = stdin()
        .lock()
        .lines()
        .map(|result| result.map(|item| filter.filter(item))?)
        .collect::<Result<_>>()?;

    let result = run_selector(cli, items).context("failed to start rmenu")?;

    if let Some(value) = result {
        println!("{}", value);
    }

    Ok(())
}
