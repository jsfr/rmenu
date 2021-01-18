use druid::{
    commands::QUIT_APP,
    im::Vector,
    lens::Identity,
    text::{ArcStr, BasicTextInput, EditAction, TextInput},
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
    fn is_selected(&self, item: &String) -> bool {
        let visible_items = self.visible_items();

        if let Some(selected_item) = visible_items.get(self.selection) {
            return selected_item == item;
        }

        false
    }

    fn insert(&mut self, chars: &String) {
        // self.selection = 0;
        self.text.push_str(chars);
    }

    fn delete_backward(&mut self) {
        // self.selection = 0;
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

    fn visible_items(&self) -> Vector<String> {
        self.items
            .clone()
            .into_iter()
            // Filter using regex to decide which items to show
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
                    Code::Tab => {}
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
            .lens(AppState::text),
    );

    root.add_child(
        List::new(|| {
            Flex::row()
                .with_child(
                    Label::new(|(_, item): &(AppState, String), _env: &_| format!("{}", item))
                        .with_font(FONT)
                        .with_text_color(FG_COLOR_NORMAL),
                )
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .background(BG_COLOR_NORMAL)
                .expand_height()
                .env_scope(|env, (data, item)| {
                    if data.is_selected(item) {
                        env.set(BG_COLOR_NORMAL, env.get(BG_COLOR_SELECTION));
                        env.set(FG_COLOR_NORMAL, env.get(FG_COLOR_SELECTION))
                    }
                })
        })
        .horizontal()
        .lens(Identity.map(
            |d: &AppState| (d.clone(), d.items.clone()),
            |d: &mut AppState, (new_d, _): (AppState, Vector<String>)| {
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
    /// The items to select between, default to stdin
    #[structopt(parse(try_from_str = parse_vector_string))]
    items: Option<Vector<String>>,
}

fn main() -> Result<(), ExitFailure> {
    let args: Cli = Cli::from_args();

    let items = match args.items {
        Some(i) if i.len() > 0 => Ok(i),
        _ => io::stdin()
            .lock()
            .lines()
            .collect::<Result<Vector<String>, _>>(),
    }
    .context("failed to read items from stdin.")?;

    let display_rect = Screen::get_display_rect();

    let window_position = display_rect.origin();
    let window_size = (display_rect.width(), 30.0);

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
        .configure_env(|env, _state| {
            env.set(PROMPT, ">");

            let font_family = FontFamily::new_unchecked("JetBrains Mono");
            env.set(FONT, FontDescriptor::new(font_family).with_size(14.0));

            // TODO: Get rid of unwrap here
            env.set(BG_COLOR_SELECTION, Color::from_hex_str("#0000FF").unwrap());
            env.set(BG_COLOR_NORMAL, Color::from_hex_str("#FF0000").unwrap());
            env.set(FG_COLOR_SELECTION, Color::from_hex_str("#00FFFF").unwrap());
            env.set(FG_COLOR_NORMAL, Color::from_hex_str("#FFFFFF").unwrap());
        })
        .launch(initial_state)
        .context("Failed to start rmenu")?;

    Ok(())
}
