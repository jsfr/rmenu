use druid::{AppLauncher, WindowDesc, Widget, PlatformError, AppDelegate, DelegateCtx, WindowId, Data, Event, Env, Lens, WidgetExt, LensExt, Screen, Key, FontDescriptor, FontFamily};
use druid::widget::Label;
use druid_shell::WindowLevel;
use druid::text::{BasicTextInput, EditAction, TextInput, ArcStr};

// TODO: refactor this to be part of the configuration
const PROMPT: Key<ArcStr> = Key::new("rmenu.prompt");
const FONT: Key<FontDescriptor> = Key::new("rmenu.font_family");

#[derive(Clone, Data, Default, Lens)]
struct AppState {
    text: String
}

struct Delegate {
    input_handler: BasicTextInput
}

impl Delegate {
    pub fn new() -> Self {
        Self {
            input_handler: BasicTextInput::default()
        }
    }

    fn insert(&mut self, data: &mut AppState, chars: &String) {
        let lens = druid::lens!(AppState, text);
        let mut text = lens.get(data);
        text.push_str(chars);
        lens.put(data, text);
    }

    fn delete_backward(&mut self, data: &mut AppState) {
        let lens = druid::lens!(AppState, text);
        let mut text = lens.get(data);
        text.pop();
        lens.put(data, text);
    }
}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        _ctx: &mut DelegateCtx<'_>,
        _window_id: WindowId,
        event: Event,
        data: &mut AppState,
        _env: &Env
    ) -> Option<Event> {
        match event {
            Event::KeyDown(key_event) => {
                if let Some(edit) = self.input_handler.handle_event(&key_event) {
                    match edit {
                        EditAction::Insert(chars) | EditAction::Paste(chars) => self.insert(data, &chars),
                        EditAction::Backspace => self.delete_backward(data),
                        _ => ()
                    }
                }
                None
            }
            _ => Some(event)
        }
    }
}

fn build_ui() -> impl Widget<AppState> {
    Label::new(|text: &String, env: &Env| format!("{} {}", env.get(PROMPT), text))
        .with_font(FONT)
        .lens(AppState::text)
}

fn main() -> Result<(), PlatformError> {
    let display_rect = Screen::get_display_rect();

    let window_position = display_rect.origin();
    let window_size = (display_rect.width(), 30.0);

    let window_desc = WindowDesc::new(build_ui)
        .resizable(false)
        .show_titlebar(false)
        .set_position(window_position)
        .window_size(window_size)
        .set_level(WindowLevel::Modal);

    let initial_state = AppState::default();

    let delegate = Delegate::new();

    AppLauncher::with_window(window_desc)
        .delegate(delegate)
        .configure_env(|env, _state| {
            env.set(PROMPT, ">");
            env.set(FONT, FontDescriptor::new(FontFamily::new_unchecked("JetBrains Mono")).with_size(14.0));
        })
        .use_simple_logger()
        .launch(initial_state)?;

    Ok(())
}
