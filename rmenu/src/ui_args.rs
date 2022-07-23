use crate::cli::Cli;
use druid::{im::Vector, ArcStr, Color, FontFamily};

pub struct Args {
    pub items: Vector<(String, String)>,
    pub height: f64,
    pub input_width: f64,
    pub font_size: f64,
    pub font_family: FontFamily,
    pub prompt: ArcStr,
    pub bg_color_normal: Option<Color>,
    pub fg_color_normal: Option<Color>,
    pub bg_color_selection: Option<Color>,
    pub fg_color_selection: Option<Color>,
}

impl Args {
    pub fn from(cli: &Cli, items: Vector<(String, String)>) -> Self {
        Self {
            items,
            height: cli.height.unwrap_or(30.0),
            input_width: 100.0,
            font_size: cli.font_size.unwrap_or(11.0),
            font_family: cli.font.as_ref().map_or(FontFamily::MONOSPACE, |f| {
                FontFamily::new_unchecked(f.as_str())
            }),
            prompt: ArcStr::from(cli.prompt.unwrap_or('>').to_string()),
            bg_color_normal: cli
                .background_normal
                .as_ref()
                .and_then(|s| Color::from_hex_str(s).ok()),
            fg_color_normal: cli
                .foreground_normal
                .as_ref()
                .and_then(|s| Color::from_hex_str(s).ok()),
            bg_color_selection: cli
                .background_selection
                .as_ref()
                .and_then(|s| Color::from_hex_str(s).ok()),
            fg_color_selection: cli
                .foreground_selection
                .as_ref()
                .and_then(|s| Color::from_hex_str(s).ok()),
        }
    }
}
