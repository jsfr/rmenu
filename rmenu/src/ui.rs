use crate::{ui_args::Args, ui_data::AppData, ui_delegate::Delegate};
use druid::{
    im::Vector,
    lens::Identity,
    text::ArcStr,
    theme,
    widget::{CrossAxisAlignment, Flex, Label, List},
    AppLauncher, Color, Env, FontDescriptor, Insets, Key, Lens, LensExt, Screen, Widget, WidgetExt,
    WindowDesc,
};
use druid_shell::{Error, WindowLevel};

const PROMPT: Key<ArcStr> = Key::new("rmenu.prompt");
const FONT: Key<FontDescriptor> = Key::new("rmenu.font_family");
const BG_COLOR_SELECTION: Key<Color> = Key::new("rmenu.bg_color_selection");
const BG_COLOR_NORMAL: Key<Color> = Key::new("rmenu.bg_color_normal");
const FG_COLOR_SELECTION: Key<Color> = Key::new("rmenu.fg_color_selection");
const FG_COLOR_NORMAL: Key<Color> = Key::new("rmenu.fg_color_normal");

type ListData = (AppData, Vector<(usize, String)>);
type ListItem = (AppData, (usize, String));

fn list_lens() -> impl Lens<AppData, ListData> {
    Identity.map(
        |d: &AppData| {
            (
                d.clone(),
                d.visible_items()
                    .into_iter()
                    .enumerate()
                    .collect::<Vector<(usize, String)>>(),
            )
        },
        |d: &mut AppData, (new_d, _): (AppData, Vector<(usize, String)>)| {
            *d = new_d;
        },
    )
}

fn list_item() -> Label<ListItem> {
    Label::new(|(_, (_, item)): &(AppData, (usize, String)), _env: &_| format!("{}", item))
        .with_font(FONT)
        .with_text_color(FG_COLOR_NORMAL)
}

fn build_ui(input_width: f64) -> impl Widget<AppData> {
    let mut root = Flex::row();

    root.add_child(
        Label::new(|text: &String, env: &Env| format!("{} {}", env.get(PROMPT), text))
            .with_font(FONT)
            .with_text_color(FG_COLOR_NORMAL)
            .fix_width(input_width)
            .lens(AppData::text),
    );

    root.add_child(
        List::new(|| {
            Flex::row()
                .with_child(list_item())
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .background(BG_COLOR_NORMAL)
                .expand_height()
                .padding(Insets::uniform_xy(2.5, 0.0))
                .env_scope(|env, (data, (index, _))| {
                    if data.get_selected_index() == *index {
                        env.set(BG_COLOR_NORMAL, env.get(BG_COLOR_SELECTION));
                        env.set(FG_COLOR_NORMAL, env.get(FG_COLOR_SELECTION))
                    }
                })
        })
        .horizontal()
        .lens(list_lens()),
    );

    // TODO figure out why this is needed to trigger updates for the selection part of state and
    // get rid of it.
    root.add_child(
        Label::new(|counter: &usize, _env: &_| format!("{}", counter))
            .fix_width(0.0)
            .lens(AppData::selection),
    );

    root.background(BG_COLOR_NORMAL)
}

pub fn run_selector(args: Args) -> Result<(), Error> {
    let display_rect = Screen::get_display_rect();

    let window_position = display_rect.origin();
    let window_size = (display_rect.width(), args.height);
    let input_width = args.input_width;

    let window_desc = WindowDesc::new(move || build_ui(input_width))
        .resizable(false)
        .show_titlebar(false)
        .set_position(window_position)
        .window_size(window_size)
        .set_level(WindowLevel::Modal);

    let initial_state = AppData::new(args.items.clone());
    let delegate = Delegate::new();

    AppLauncher::with_window(window_desc)
        .delegate(delegate)
        .configure_env(move |env, _state| {
            env.set(PROMPT, args.prompt.clone());
            env.set(
                FONT,
                FontDescriptor::new(args.font_family.clone()).with_size(args.font_size),
            );
            env.set(
                BG_COLOR_NORMAL,
                args.bg_color_normal
                    .clone()
                    .unwrap_or(env.get(theme::BACKGROUND_DARK)),
            );
            env.set(
                FG_COLOR_NORMAL,
                args.fg_color_normal
                    .clone()
                    .unwrap_or(env.get(theme::FOREGROUND_DARK)),
            );
            env.set(
                BG_COLOR_SELECTION,
                args.bg_color_selection
                    .clone()
                    .unwrap_or(env.get(theme::BACKGROUND_LIGHT)),
            );
            env.set(
                FG_COLOR_SELECTION,
                args.fg_color_selection
                    .clone()
                    .unwrap_or(env.get(theme::FOREGROUND_LIGHT)),
            );
        })
        .launch(initial_state)?;

    Ok(())
}
