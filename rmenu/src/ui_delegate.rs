use crate::ui_data::AppData;
use druid::{
    commands::QUIT_APP,
    text::{BasicTextInput, EditAction, TextInput},
    AppDelegate, Code, DelegateCtx, Env, Event, WindowId,
};

pub struct Delegate {
    input_handler: BasicTextInput,
}

impl Delegate {
    pub fn new() -> Self {
        Self {
            input_handler: BasicTextInput::default(),
        }
    }
}

impl AppDelegate<AppData> for Delegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx<'_>,
        _window_id: WindowId,
        event: Event,
        data: &mut AppData,
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
                        if let Some(item) = data.get_selected_item() {
                            // TODO: don't print here but instead store/return the item
                            println!("{}", item);
                        }
                        ctx.submit_command(QUIT_APP);
                    }
                    Code::Tab => data.complete(),
                    _ => {
                        if let Some(edit) = self.input_handler.handle_event(&key_event) {
                            match edit {
                                EditAction::Insert(chars) | EditAction::Paste(chars) => {
                                    data.insert(chars.as_str())
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
