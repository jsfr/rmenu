use druid::im::Vector;
use failure::Error;
use structopt::StructOpt;

fn parse_vector_string(src: &str) -> Result<Vector<String>, Error> {
    Ok(src.lines().map(|str| str.to_owned()).collect::<Vector<_>>())
}

/// Shows a selector to fuzzy select between a list of items
#[derive(StructOpt)]
#[structopt(name = "rmenu", version = "0.1.0")]
pub struct Cli {
    /// The font used by the selector
    #[structopt(long)]
    pub font: Option<String>,

    /// The size of the font,
    #[structopt(long)]
    pub font_size: Option<f64>,

    /// The character or string used as prompt
    #[structopt(long)]
    pub prompt: Option<char>,

    /// The normal background color in RGB format
    #[structopt(long)]
    pub background_normal: Option<String>,

    /// The normal foreground color in RGB format
    #[structopt(long)]
    pub foreground_normal: Option<String>,

    /// The selection background color in RGB format
    #[structopt(long)]
    pub background_selection: Option<String>,

    /// The selection foreground color in RGB format
    #[structopt(long)]
    pub foreground_selection: Option<String>,

    /// The height of the bar in pixels
    #[structopt(long)]
    pub height: Option<f64>,

    /// The items to select between, default to stdin
    #[structopt(parse(try_from_str = parse_vector_string))]
    pub items: Option<Vector<String>>,
}
