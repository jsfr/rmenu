use druid::{im::Vector, Data, Lens};

use crate::Item;

#[derive(Clone, Data, Lens)]
pub struct AppData {
    text: String,
    items: Vector<Item>,
    selection: usize,
}

impl AppData {
    pub fn new(items: Vector<Item>) -> Self {
        Self {
            text: String::from(""),
            items,
            selection: 0,
        }
    }

    pub fn insert(&mut self, chars: &str) {
        self.selection = 0;
        self.text.push_str(chars);
    }

    pub fn delete_backward(&mut self) {
        self.selection = 0;
        self.text.pop();
    }

    pub fn next(&mut self) {
        let visible_items = self.visible_items();

        if self.selection < visible_items.len() - 1 {
            self.selection += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selection > 0 {
            self.selection -= 1;
        }
    }

    pub fn complete(&mut self) {
        let visible_items = self.visible_items();
        let selection = self.selection;

        if let Some(item) = visible_items.get(selection) {
            self.text = item.key.to_string();
        }
    }

    pub fn visible_items(&self) -> Vector<Item> {
        self.items
            .iter()
            // Filter using regex to decide which items to show
            .filter(|Item{key, ..}| {
                key.to_ascii_lowercase()
                    .contains(self.text.to_ascii_lowercase().as_str())
            })
            .cloned()
            .collect()
    }

    pub fn get_selected_item(&self) -> Option<Item> {
        let items = self.visible_items();
        let index = self.selection;

        items.get(index).cloned()
    }

    pub fn get_selected_index(&self) -> usize {
        self.selection
    }
}
