use druid::{
    commands::QUIT_APP,
    im::Vector,
    lens::Identity,
    text::{ArcStr, BasicTextInput, EditAction, TextInput},
    theme,
    widget::{CrossAxisAlignment, Flex, Label, List},
    AppDelegate, AppLauncher, Code, Color, Data, DelegateCtx, Env, Event, FontDescriptor,
    FontFamily, Key, Lens, LensExt, Screen, Widget, WidgetExt, WindowDesc, WindowId,
};
use druid_shell::WindowLevel;
use exitfailure::ExitFailure;
use failure::{Error, ResultExt};
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

const PROMPT: Key<ArcStr> = Key::new("rmenu.prompt");
const FONT: Key<FontDescriptor> = Key::new("rmenu.font_family");
const BG_COLOR_SELECTION: Key<Color> = Key::new("rmenu.bg_color_selection");
const BG_COLOR_NORMAL: Key<Color> = Key::new("rmenu.bg_color_normal");
const FG_COLOR_SELECTION: Key<Color> = Key::new("rmenu.fg_color_selection");
const FG_COLOR_NORMAL: Key<Color> = Key::new("rmenu.fg_color_normal");

#[derive(Clone, Data, Lens)]
struct AppState {
    text: String,
    items: Vector<String>,
    selection: usize,
}

impl AppState {
    fn insert(&mut self, chars: &String) {
        self.selection = 0;
        self.text.push_str(chars);
    }

    fn delete_backward(&mut self) {
        self.selection = 0;
        self.text.pop();
    }

    fn next(&mut self) {
        let visible_items = self.visible_items();

        if self.selection < visible_items.len() - 1 {
            self.selection += 1;
        }
    }

    fn previous(&mut self) {
        if self.selection > 0 {
            self.selection -= 1;
        }
    }

    fn complete(&mut self) {
        let visible_items = self.visible_items();
        let selection = self.selection;

        if let Some(item) = visible_items.get(selection) {
            self.text = item.clone();
        }
    }

    fn visible_items(&self) -> Vector<String> {
        self.items
            .clone()
            .into_iter()
            // Filter using regex to decide which items to show
            .filter(|item| {
                item.to_ascii_lowercase()
                    .contains(self.text.to_ascii_lowercase().as_str())
            })
            .collect()
    }
}

struct Delegate {
    input_handler: BasicTextInput,
}

impl Delegate {
    pub fn new() -> Self {
        Self {
            input_handler: BasicTextInput::default(),
        }
    }
}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx<'_>,
        _window_id: WindowId,
        event: Event,
        data: &mut AppState,
        _env: &Env,
    ) -> Option<Event> {
        match event {
            Event::KeyDown(key_event) => {
                match key_event.code {
                    Code::Escape => ctx.submit_command(QUIT_APP),
                    Code::ArrowLeft => {
                        data.previous();
                    }
                    Code::ArrowRight => {
                        data.next();
                    }
                    Code::Enter => {
                        if let Some(item) = data.visible_items().get(data.selection) {
                            println!("{}", item);
                        }
                        ctx.submit_command(QUIT_APP);
                    }
                    Code::Tab => data.complete(),
                    _ => {
                        if let Some(edit) = self.input_handler.handle_event(&key_event) {
                            match edit {
                                EditAction::Insert(chars) | EditAction::Paste(chars) => {
                                    data.insert(&chars)
                                }
                                EditAction::Backspace => data.delete_backward(),
                                _ => {}
                            }
                        }
                    }
                }
                None
            }
            _ => Some(event),
        }
    }
}

fn build_ui() -> impl Widget<AppState> {
    let mut root = Flex::row();

    root.add_child(
        Label::new(|text: &String, env: &Env| format!("{} {}", env.get(PROMPT), text))
            .with_font(FONT)
            .with_text_color(FG_COLOR_NORMAL)
            .lens(AppState::text),
    );

    root.add_child(
        List::new(|| {
            Flex::row()
                .with_child(
                    Label::new(|(_, (_, item)): &(AppState, (usize, String)), _env: &_| {
                        format!("{}", item)
                    })
                    .with_font(FONT)
                    .with_text_color(FG_COLOR_NORMAL),
                )
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .background(BG_COLOR_NORMAL)
                .expand_height()
                .env_scope(|env, (data, (index, _))| {
                    if data.selection == *index {
                        env.set(BG_COLOR_NORMAL, env.get(BG_COLOR_SELECTION));
                        env.set(FG_COLOR_NORMAL, env.get(FG_COLOR_SELECTION))
                    }
                })
        })
        .horizontal()
        .lens(Identity.map(
            |d: &AppState| {
                (
                    d.clone(),
                    d.visible_items()
                        .into_iter()
                        .enumerate()
                        .collect::<Vector<(usize, String)>>(),
                )
            },
            |d: &mut AppState, (new_d, _): (AppState, Vector<(usize, String)>)| {
                *d = new_d;
            },
        )),
    );

    // TODO figure out why this is needed to trigger updates for the selection part of state and
    // get rid of it.
    root.add_child(
        Label::new(|counter: &usize, _env: &_| format!("{}", counter)).lens(AppState::selection),
    );

    root.background(BG_COLOR_NORMAL)
}

fn parse_vector_string(src: &str) -> Result<Vector<String>, Error> {
    let items = src.lines().map(|str| str.to_owned()).collect::<Vector<_>>();

    Ok(items)
}

/// Shows a selector to fuzzy select between a list of items
#[derive(StructOpt)]
#[structopt(name = "rmenu", version = "0.1.0")]
struct Cli {
    /// The font used by the selector
    #[structopt(long)]
    font: Option<String>,

    /// The size of the font,
    #[structopt(long)]
    font_size: Option<f64>,

    /// The character or string used as prompt
    #[structopt(long)]
    prompt: Option<char>,

    /// The normal background color in RGB format
    #[structopt(long)]
    background_normal: Option<String>,

    /// The normal foreground color in RGB format
    #[structopt(long)]
    foreground_normal: Option<String>,

    /// The selection background color in RGB format
    #[structopt(long)]
    background_selection: Option<String>,

    /// The selection foreground color in RGB format
    #[structopt(long)]
    foreground_selection: Option<String>,

    /// The height of the bar in pixels
    #[structopt(long)]
    height: Option<f64>,

    /// The items to select between, default to stdin
    #[structopt(parse(try_from_str = parse_vector_string))]
    items: Option<Vector<String>>,
}

fn main() -> Result<(), ExitFailure> {
    let args: Cli = Cli::from_args();

    let items = match args.items {
        Some(ref i) if i.len() > 0 => Ok(i.clone()),
        _ => io::stdin()
            .lock()
            .lines()
            .collect::<Result<Vector<String>, _>>(),
    }
    .context("failed to read items from stdin.")?;

    let display_rect = Screen::get_display_rect();

    let window_position = display_rect.origin();
    let window_size = (display_rect.width(), args.height.unwrap_or(30.0));

    let window_desc = WindowDesc::new(build_ui)
        .resizable(false)
        .show_titlebar(false)
        .set_position(window_position)
        .window_size(window_size)
        .set_level(WindowLevel::Modal);

    let initial_state = AppState {
        text: String::from(""),
        items,
        selection: 0,
    };

    let delegate = Delegate::new();

    AppLauncher::with_window(window_desc)
        .delegate(delegate)
        .configure_env(move |env, _state| {
            let prompt = args.prompt.unwrap_or('>');
            env.set(PROMPT, prompt.to_string());

            let font_family = args.font.as_ref().map_or(FontFamily::MONOSPACE, |f| {
                FontFamily::new_unchecked(f.as_str())
            });
            let font_size = args.font_size.unwrap_or(11.0);
            env.set(FONT, FontDescriptor::new(font_family).with_size(font_size));

            let bg_color_normal = args
                .background_normal
                .as_ref()
                .and_then(|s| Color::from_hex_str(s).ok())
                .unwrap_or(env.get(theme::BACKGROUND_DARK));
            env.set(BG_COLOR_NORMAL, bg_color_normal);

            let fg_color_normal = args
                .foreground_normal
                .as_ref()
                .and_then(|s| Color::from_hex_str(s).ok())
                .unwrap_or(env.get(theme::FOREGROUND_DARK));
            env.set(FG_COLOR_NORMAL, fg_color_normal);

            let bg_color_selection = args
                .background_selection
                .as_ref()
                .and_then(|s| Color::from_hex_str(s).ok())
                .unwrap_or(env.get(theme::BACKGROUND_LIGHT));
            env.set(BG_COLOR_SELECTION, bg_color_selection);

            let fg_color_selection = args
                .foreground_selection
                .as_ref()
                .and_then(|s| Color::from_hex_str(s).ok())
                .unwrap_or(env.get(theme::FOREGROUND_LIGHT));
            env.set(FG_COLOR_SELECTION, fg_color_selection);
        })
        .launch(initial_state)
        .context("Failed to start rmenu")?;

    Ok(())
}
