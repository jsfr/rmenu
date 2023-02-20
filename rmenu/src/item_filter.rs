use std::sync::Arc;

use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum ItemFilters {
    Contains,
    Substring,
}

impl From<ItemFilters> for Arc<dyn ItemFilter> {
    fn from(value: ItemFilters) -> Self {
        match value {
            ItemFilters::Contains => Arc::new(ContainsFilter {}),
            ItemFilters::Substring => Arc::new(SubstringFilter {}),
        }
    }
}

pub trait ItemFilter {
    fn filter(&self, filter: &str, item: &str) -> bool;
}

pub struct ContainsFilter {}
impl ItemFilter for ContainsFilter {
    fn filter(&self, filter: &str, item: &str) -> bool {
        item.to_ascii_lowercase()
            .contains(filter.to_ascii_lowercase().as_str())
    }
}

pub struct SubstringFilter {}
impl ItemFilter for SubstringFilter {
    fn filter(&self, filter: &str, item: &str) -> bool {
        let mut item_chars = item.chars();

        'outer: for a in filter.chars() {
            loop {
                match item_chars.next() {
                    Some(b) => {
                        if a.to_ascii_lowercase() == b.to_ascii_lowercase() {
                            continue 'outer;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }

        true
    }
}
