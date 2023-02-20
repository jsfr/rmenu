use anyhow::{Context, Error};
use clap::Parser;
use druid::{ArcStr, Color, FontFamily};

use crate::item_filter::ItemFilters;

fn parse_color(src: &str) -> Result<Color, Error> {
    Color::from_hex_str(src).context("format of color string should be '[#]RGB' or '[#]RRGGBB'")
}

#[allow(clippy::unnecessary_wraps)]
fn parse_font(src: &str) -> Result<FontFamily, Error> {
    Ok(if src.is_empty() {
        FontFamily::MONOSPACE
    } else {
        FontFamily::new_unchecked(src)
    })
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The font used by the selector
    #[arg(long = "font", value_parser = parse_font, default_value = "")]
    pub font_family: FontFamily,

    /// The size of the font,
    #[arg(long, default_value_t = 13.0)]
    pub font_size: f64,

    /// The character or string used as prompt
    #[arg(long, default_value = ">")]
    pub prompt: ArcStr,

    /// The normal background color in RGB format
    #[arg(long, value_parser = parse_color, default_value = "3a3a3a")]
    pub background_normal: Color,

    /// The normal foreground color in RGB format
    #[arg(long, value_parser = parse_color, default_value = "d0d0d0")]
    pub foreground_normal: Color,

    /// The selection background color in RGB format
    #[arg(long, value_parser = parse_color, default_value = "85add4")]
    pub background_selection: Color,

    /// The selection foreground color in RGB format
    #[arg(long, value_parser = parse_color, default_value = "d0d0d0")]
    pub foreground_selection: Color,

    /// The height of the bar in pixels
    #[arg(long, default_value_t = 30.0)]
    pub height: f64,

    /// The filter used to filter items against the search string
    #[arg(long, value_enum, default_value_t = ItemFilters::Substring)]
    pub item_filter: ItemFilters,

    /// A jaq filter to apply to items for showing.
    /// When set the items are treated as JSON objects
    #[arg(long)]
    pub json_filter: Option<String>,

    #[arg(skip = 100.0)]
    pub input_width: f64,
}
