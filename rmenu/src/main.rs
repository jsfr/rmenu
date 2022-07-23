mod cli;
mod ui;
mod ui_args;
mod ui_data;
mod ui_delegate;

use crate::{cli::Cli, ui::run_selector, ui_args::Args};
use anyhow::{Context, Result, bail};
use clap::Parser;
use druid::im::Vector;
use serde_json::Value;
use std::io::{prelude::*, stdin};
use jaq_core::{parse, Ctx, Definitions, Val};

fn jaq_filter(item: String, filter: &str) -> Result<(String, String)> {
    let input: Value = serde_json::from_str(&item).context("failed to parse item as json.")?;
    // start out only from core filters,
    // which do not include filters in the standard library
    // such as `map`, `select` etc.
    let defs = Definitions::core();

    // parse the filter in the context of the given definitions
    let mut errs = Vec::new();
    let f = parse::parse(filter, parse::main()).0.unwrap();
    let f = defs.finish(f, Vec::new(), &mut errs);

    // iterator over the output values
    let mut out = f.run(Ctx::new(), Val::from(input));

    match out.next() {
        Some(Ok(val)) => Ok((val.to_string().trim_matches('"').to_string(), item)),
        _ => bail!("found no value when applying filter."),
    }
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    let items = match cli.items {
        Some(ref i) if !i.is_empty() => i.clone(),
        _ => stdin()
            .lock()
            .lines()
            .collect::<Result<Vector<String>, _>>()
            .context("failed to read items from stdin.")?
    }
        .into_iter()
        .map(|item| match &cli.filter {
            None => Ok((item.clone(), item)),
            Some(filter) => jaq_filter(item, filter)
        })
        .collect::<Result<Vector<(String, String)>>>()?;

    let ui_args: Args = Args::from(&cli, items);

    run_selector(ui_args).context("failed to start rmenu")?;

    Ok(())
}
