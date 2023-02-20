use anyhow::{bail, Context, Result};
use jaq_core::{parse, Ctx, Definitions, RcIter, Val};
use serde_json::Value;

use crate::Item;

pub struct Filter {
    filter: Option<String>,
}

impl Filter {
    pub fn new(filter: &Option<String>) -> Self {
        Self {
            filter: filter.clone(),
        }
    }

    pub fn filter(&self, item: String) -> Result<Item> {
        match &self.filter {
            Some(filter) => {
                let input: Value =
                    serde_json::from_str(&item).context("failed to parse item as json.")?;

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
            None => Ok(Item {
                key: item.as_str().into(),
                value: item.into(),
            }),
        }
    }
}
