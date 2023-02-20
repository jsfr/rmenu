use std::sync::{Arc, Mutex};

use crate::item_filter::ItemFilter;
use crate::item_label::ItemLabelExt;
use egui::{
    CentralPanel, Color32, Context, Event, FontFamily, FontId, Frame, Key, Margin, Modifiers,
    RichText, Style, Ui, Vec2, Visuals,
};

use crate::item::Item;

pub struct AppColors {
    pub foreground_normal: Color32,
    pub background_normal: Color32,
    pub foreground_selection: Color32,
    pub background_selection: Color32,
}

pub struct AppFont {
    pub size: f32,
    pub family: FontFamily,
}

pub struct Selector {
    text: String,
    items: Vec<Item>,
    selection: usize,
    prompt: String,
    item_filter: Arc<dyn ItemFilter>,
    colors: AppColors,
    result: Arc<Mutex<Option<String>>>,
}

fn clamp(low: usize, value: usize, high: usize) -> usize {
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

impl Selector {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        items: Vec<Item>,
        item_filter: Arc<dyn ItemFilter>,
        prompt: String,
        colors: AppColors,
        font: AppFont,
        result: Arc<Mutex<Option<String>>>,
    ) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let visuals = Visuals {
            panel_fill: colors.background_normal,
            override_text_color: Some(colors.foreground_normal),
            ..Default::default()
        };

        let style = Style {
            override_font_id: Some(FontId::new(font.size, font.family)),
            visuals,
            ..Default::default()
        };

        cc.egui_ctx.set_style(style);

        Self {
            text: String::new(),
            items,
            prompt,
            selection: 0,
            item_filter,
            colors,
            result,
        }
    }

    fn visible_items(&self) -> Vec<Item> {
        //TODO: Can we memoize this based on the search text
        self.items
            .iter()
            .filter(|Item { key, .. }| self.item_filter.filter(&self.text, key))
            .cloned()
            .collect()
    }

    fn is_selection(&self, index: usize) -> bool {
        self.selection == index
    }

    fn selected_item_value(&self) -> Option<String> {
        self.visible_items()
            .get(self.selection)
            .map(|item| item.value.clone())
    }

    fn handle_input(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        let visible_items = self.visible_items();

        ui.input(|input| {
            for event in &input.events {
                match event {
                    Event::Text(text) => {
                        self.selection = 0;
                        self.text += text;
                    }
                    Event::Key {
                        key,
                        pressed: true,
                        repeat: _,
                        modifiers: Modifiers::NONE,
                    } => match key {
                        Key::Backspace => {
                            self.selection = 0;
                            self.text.pop();
                        }
                        Key::Escape => {
                            frame.close();
                        }
                        Key::Enter => {
                            *self.result.lock().unwrap() = self.selected_item_value();
                            frame.close();
                        }
                        Key::ArrowLeft => {
                            let max = visible_items.len() - 1;
                            self.selection = self
                                .selection
                                .checked_sub(1)
                                .map(|i| clamp(0, i, max))
                                .unwrap_or(0);
                        }
                        Key::ArrowRight => {
                            let max = visible_items.len() - 1;
                            self.selection = self
                                .selection
                                .checked_add(1)
                                .map(|i| clamp(0, i, max))
                                .unwrap_or(max);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        });
    }
}

impl eframe::App for Selector {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let panel_frame = Frame {
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };
        CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            let mut spacing = ui.spacing_mut();
            spacing.item_spacing = Vec2::ZERO;
            spacing.window_margin = Margin::same(0.0);

            self.handle_input(ui, frame);

            ui.horizontal_centered(|ui| {
                ui.horizontal_centered(|ui| {
                    ui.set_min_width(100.0);
                    ui.label(format!("{} {}", self.prompt, self.text));
                });

                let visible_items = self.visible_items();
                for (index, item) in visible_items.iter().enumerate() {
                    ui.horizontal_centered(|ui| {
                        let text = RichText::new(&item.key);
                        let (foreground_color, background_color) = if self.is_selection(index) {
                            (
                                Some(self.colors.foreground_selection),
                                Some(self.colors.background_selection),
                            )
                        } else {
                            (None, None)
                        };

                        ui.add_space(5.0);
                        ui.item_label(text, foreground_color, background_color);
                        ui.add_space(5.0);
                    });
                }
            });
        });
    }
}
