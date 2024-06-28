use std::fs;

use anyhow::{bail, Context, Error};
use clap::Parser;
use egui::{Color32, FontData, FontDefinitions};

use font_kit::{handle::Handle, source::SystemSource};

use crate::item_filter::ItemFilters;

const fn hex_from_ascii_byte(b: u8) -> Result<u8, u8> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        _ => Err(b),
    }
}

fn parse_color(src: &str) -> Result<Color32, Error> {
    let mut four_bit_channels = match src.as_bytes() {
        &[b'#', r, g, b] | &[r, g, b] => [r, r, g, g, b, b, b'f', b'f'],
        &[b'#', r, g, b, a] | &[r, g, b, a] => [r, r, g, g, b, b, a, a],
        &[b'#', r0, r1, g0, g1, b0, b1] | &[r0, r1, g0, g1, b0, b1] => {
            [r0, r1, g0, g1, b0, b1, b'f', b'f']
        }
        &[b'#', r0, r1, g0, g1, b0, b1, a0, a1] | &[r0, r1, g0, g1, b0, b1, a0, a1] => {
            [r0, r1, g0, g1, b0, b1, a0, a1]
        }
        _ => bail!("format of color string should be '[#]RGB' or '[#]RRGGBB'"),
    };

    // convert to hex in-place
    // this is written without a for loop to satisfy `const`
    let mut i = 0;
    while i < four_bit_channels.len() {
        let ascii = four_bit_channels[i];
        let as_hex = match hex_from_ascii_byte(ascii) {
            Ok(hex) => hex,
            Err(byte) => bail!("byte was not a proper hex field {byte}"),
        };
        four_bit_channels[i] = as_hex;
        i += 1;
    }

    let [r0, r1, g0, g1, b0, b1, a0, a1] = four_bit_channels;

    Ok(Color32::from_rgba_premultiplied(
        r0 << 4 | r1,
        g0 << 4 | g1,
        b0 << 4 | b1,
        a0 << 4 | a1,
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn parse_font(src: &str) -> Result<FontDefinitions, Error> {
    let mut definitions = FontDefinitions::default();

    if !src.is_empty() {
        let font_handle = SystemSource::new()
            .select_by_postscript_name(src)
            .with_context(|| format!("failed to find a font named {src}"))?;

        let font_data = match font_handle {
            Handle::Path {
                path,
                font_index: 0,
            } => {
                let bytes = fs::read(path).context("failed to read font data from path")?;
                FontData::from_owned(bytes)
            }
            Handle::Memory {
                bytes,
                font_index: 0,
            } => FontData::from_owned(bytes.to_vec()),
            _ => bail!("failed to find a single font"),
        };

        definitions
            .font_data
            .insert("custom_font".to_string(), font_data);

        definitions
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "custom_font".to_owned());

        definitions
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "custom_font".to_owned());
    }

    Ok(definitions)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The font used by the selector
    #[arg(long = "font", id = "FONT", value_parser = parse_font, default_value = "")]
    pub font_definitions: FontDefinitions,

    /// The size of the font,
    #[arg(long, default_value_t = 13.0)]
    pub font_size: f32,

    /// The character or string used as prompt
    #[arg(long, default_value = ">")]
    pub prompt: String,

    /// The normal background color in RGB format
    #[arg(long, value_parser = parse_color, default_value = "3a3a3a")]
    pub background_normal: Color32,

    /// The normal foreground color in RGB format
    #[arg(long, value_parser = parse_color, default_value = "d0d0d0")]
    pub foreground_normal: Color32,

    /// The selection background color in RGB format
    #[arg(long, value_parser = parse_color, default_value = "85add4")]
    pub background_selection: Color32,

    /// The selection foreground color in RGB format
    #[arg(long, value_parser = parse_color, default_value = "d0d0d0")]
    pub foreground_selection: Color32,

    /// The height of the bar in pixels
    #[arg(long, default_value_t = 30.0)]
    pub height: f32,

    /// The filter used to filter items against the search string
    #[arg(long, value_enum, default_value_t = ItemFilters::Substring)]
    pub item_filter: ItemFilters,

    /// A jql filter to apply to items for showing.
    /// When set the items are treated as JSON objects
    #[arg(long)]
    pub jql_filter: Option<String>,

    #[arg(skip = 100.0)]
    pub input_min_width: f32,
}
