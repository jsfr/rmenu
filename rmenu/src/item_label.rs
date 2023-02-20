use egui::{
    Color32, Label, Response, Stroke, Ui, Vec2, Widget, WidgetInfo, WidgetText, WidgetType,
};

pub trait ItemLabelExt {
    fn item_label(
        &mut self,
        text: impl Into<WidgetText>,
        foreground_color: Option<Color32>,
        background_color: Option<Color32>,
    ) -> Response;
}

impl ItemLabelExt for Ui {
    fn item_label(
        &mut self,
        text: impl Into<WidgetText>,
        foreground_color: Option<Color32>,
        background_color: Option<Color32>,
    ) -> Response {
        ItemLabel::new(text, foreground_color, background_color).ui(self)
    }
}

struct ItemLabel {
    text: WidgetText,
    foreground_color: Option<Color32>,
    background_color: Option<Color32>,
}

impl ItemLabel {
    fn new(
        text: impl Into<WidgetText>,
        foreground_color: Option<Color32>,
        background_color: Option<Color32>,
    ) -> Self {
        Self {
            text: text.into(),
            foreground_color,
            background_color,
        }
    }
}

impl Widget for ItemLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let text = match self.foreground_color {
            Some(color) => self.text.color(color),
            None => self.text,
        };

        let label = Label::new(text);

        let (pos, text_galley, response) = label.layout_in_ui(ui);

        let painter = ui.painter();

        if let Some(color) = self.background_color {
            let max = ui.max_rect();

            let mut rect = response.rect;
            rect.extend_with_y(max.min.y);
            rect.extend_with_y(max.max.y);
            rect = rect.expand2(Vec2::new(5.0, 0.0));

            painter.rect_filled(rect, 0.0, color);
        }

        response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, text_galley.text()));

        if ui.is_rect_visible(response.rect) {
            let response_color = ui.style().interact(&response).text_color();

            let underline = if response.has_focus() || response.highlighted() {
                Stroke::new(1.0, response_color)
            } else {
                Stroke::NONE
            };

            let override_text_color = if text_galley.galley_has_color {
                None
            } else {
                Some(response_color)
            };

            painter.add(eframe::epaint::TextShape {
                pos,
                galley: text_galley.galley,
                override_text_color,
                underline,
                angle: 0.0,
            });
        }

        response
    }
}
