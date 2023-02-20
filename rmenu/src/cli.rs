use anyhow::{bail, Error};
use clap::Parser;
use egui::{Color32, FontFamily};

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
fn parse_font(src: &str) -> Result<FontFamily, Error> {
    // TODO: Write this function
    // dirs::font_dir()
    // Ok(if src.is_empty() {
    //     FontFamily::default()
    // } else {
    //     FontFamily::new_unchecked(src)
    // })
    Ok(FontFamily::default())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The font used by the selector
    #[arg(long = "font", value_parser = parse_font, default_value = "")]
    pub font_family: FontFamily,

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

    /// A jaq filter to apply to items for showing.
    /// When set the items are treated as JSON objects
    #[arg(long)]
    pub json_filter: Option<String>,

    #[arg(skip = 100.0)]
    pub input_min_width: f32,
}
