use std::{
    io::BufRead,
    sync::mpsc::{self, Receiver, Sender},
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cli::Cli;
use cocoa::appkit::NSScreen;
use cocoa::base::nil;

use egui::{Pos2, Vec2};
use filter::Filter;
use item::Item;
use selector::{AppColors, AppFont, Selector};

mod cli;
mod filter;
mod item;
mod item_filter;
mod item_label;
mod selector;

fn get_main_screen_width() -> f32 {
    let frame = unsafe {
        let object = NSScreen::mainScreen(nil);
        object.frame()
    };

    let size = frame.size;

    size.width as f32
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    let filter = Filter::new(&cli.jql_filter);

    // TODO: should this be a im::Vector
    let items: Vec<Item> = std::io::stdin()
        .lock()
        .lines()
        .map(|result| result.map(|item| filter.to_item(item))?)
        .collect::<Result<_>>()?;

    let width = get_main_screen_width();

    let native_options = eframe::NativeOptions {
        decorated: false,
        initial_window_size: Some(Vec2::new(width, cli.height)),
        resizable: false,
        always_on_top: true,
        initial_window_pos: Some(Pos2::new(0.0, 0.0)),
        ..Default::default()
    };

    let app_colors = AppColors {
        foreground_normal: cli.foreground_normal,
        foreground_selection: cli.foreground_selection,
        background_normal: cli.background_normal,
        background_selection: cli.background_selection,
    };

    let app_font = AppFont {
        size: cli.font_size,
        definitions: cli.font_definitions,
    };

    let (tx, rx): (Sender<Option<String>>, Receiver<Option<String>>) = mpsc::channel();

    eframe::run_native(
        "rmenu",
        native_options,
        Box::new(|cc| {
            Box::new(Selector::new(
                cc,
                items,
                cli.item_filter.into(),
                cli.prompt,
                app_colors,
                app_font,
                tx,
            ))
        }),
    )
    .map_err(|err| anyhow!("{err}"))?;

    if let Some(item) = rx.try_recv().context("failed to get result")? {
        println!("{item}");
    }

    Ok(())
}
