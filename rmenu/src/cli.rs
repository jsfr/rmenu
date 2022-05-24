use anyhow::Error;
use clap::Parser;
use druid::im::Vector;

fn parse_vector_string(src: &str) -> Result<Vector<String>, Error> {
    Ok(src.lines().map(String::from).collect::<Vector<_>>())
}

/// Shows a selector to fuzzy select between a list of items
#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Jens Fredskov <jsfr@users.noreply.github.com>"
)]
pub struct Cli {
    /// The font used by the selector
    #[clap(long)]
    pub font: Option<String>,

    /// The size of the font,
    #[clap(long)]
    pub font_size: Option<f64>,

    /// The character or string used as prompt
    #[clap(long)]
    pub prompt: Option<char>,

    /// The normal background color in RGB format
    #[clap(long)]
    pub background_normal: Option<String>,

    /// The normal foreground color in RGB format
    #[clap(long)]
    pub foreground_normal: Option<String>,

    /// The selection background color in RGB format
    #[clap(long)]
    pub background_selection: Option<String>,

    /// The selection foreground color in RGB format
    #[clap(long)]
    pub foreground_selection: Option<String>,

    /// The height of the bar in pixels
    #[clap(long)]
    pub height: Option<f64>,

    /// The items to select between, default to stdin
    #[clap(parse(try_from_str = parse_vector_string))]
    pub items: Option<Vector<String>>,
}
