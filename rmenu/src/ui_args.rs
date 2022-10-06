use crate::cli::Cli;
use druid::{im::Vector, ArcStr, Color, FontFamily};

pub struct Args {
    pub items: Vector<(String, String)>,
    pub height: f64,
    pub input_width: f64,
    pub font_size: f64,
    pub font_family: FontFamily,
    pub prompt: ArcStr,
    pub background_normal: Color,
    pub foreground_normal: Color,
    pub background_selection: Color,
    pub foreground_selection: Color,
}

impl Args {
    pub fn from(cli: &Cli, items: Vector<(String, String)>) -> Self {
        Self {
            items,
            height: cli.height,
            input_width: cli.input_width,
            font_size: cli.font_size,
            font_family: cli.font_family.clone(),
            prompt: cli.prompt.clone(),
            background_normal: cli.background_normal.clone(),
            foreground_normal: cli.foreground_normal.clone(),
            background_selection: cli.background_selection.clone(),
            foreground_selection: cli.foreground_selection.clone(),
        }
    }
}
