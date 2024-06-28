use anyhow::{Context, Result};
use jql_runner::runner;
use serde_json::Value;

use crate::item::Item;

pub struct Filter {
    filter: Option<String>,
}

impl Filter {
    pub fn new(filter: &Option<String>) -> Self {
        Self {
            filter: filter.clone(),
        }
    }

    pub fn to_item(&self, item: String) -> Result<Item> {
        match &self.filter {
            Some(filter) => {
                let input: Value =
                    serde_json::from_str(&item).context("failed to parse item as json")?;

                let value = runner::raw(&filter, &input).context("failed to apply filter on item")?;

                Ok(Item {
                    key: value.to_string().trim_matches('"').to_string(),
                    value: item
                })
            }
            None => Ok(Item {
                key: item.clone(),
                value: item,
            }),
        }
    }
}
