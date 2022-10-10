mod cli;
mod ui;
mod ui_args;
mod ui_data;
mod ui_delegate;

use crate::{cli::Cli, ui::run_selector, ui_args::Args};
use anyhow::{bail, Context, Result};
use clap::Parser;
use druid::{im::Vector, ArcStr};
use jaq_core::{parse, Ctx, Definitions, RcIter, Val};
use serde_json::Value;
use std::io::{prelude::*, stdin};
use ui_args::Item;

fn jaq_filter(item: String, filter: &str) -> Result<Item> {
    let input: Value = serde_json::from_str(&item).context("failed to parse item as json.")?;
    // start out only from core filters,
    // which do not include filters in the standard library
    // such as `map`, `select` etc.
    let defs = Definitions::core();

    // parse the filter in the context of the given definitions
    let mut errs = Vec::new();
    let f = parse::parse(filter, parse::main()).0.unwrap();
    let f = defs.finish(f, Vec::new(), &mut errs);

    let inputs = RcIter::new(core::iter::empty());

    // iterator over the output values
    let mut out = f.run(Ctx::new([], &inputs), Val::from(input));
    match out.next() {
        Some(Ok(val)) => Ok(Item {
            key: val.to_string().trim_matches('"').into(),
            value: item.into(),
        }),
        _ => bail!("found no value when applying filter."),
    }
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    let items: Result<Vector<Item>> = match cli.items {
        Some(ref i) if !i.is_empty() => i.clone(),
        _ => stdin()
            .lock()
            .lines()
            .collect::<Result<Vector<String>, _>>()
            .context("failed to read items from stdin.")?,
    }
    .into_iter()
    .map(|item| match &cli.filter {
        None => {
            let value: ArcStr = item.into();
            Ok(Item {
                key: value.clone(),
                value,
            })
        }
        Some(filter) => jaq_filter(item, filter),
    })
    .collect();

    let ui_args: Args = Args::from(&cli, items?);

    let result = run_selector(ui_args).context("failed to start rmenu")?;

    if let Some(value) = result {
        println!("{}", value)
    }

    Ok(())
}
