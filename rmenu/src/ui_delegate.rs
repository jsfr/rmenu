use crate::ui_data::AppData;
use druid::{
    commands::QUIT_APP, keyboard_types::Key, AppDelegate, DelegateCtx, Env, Event, WindowId,
};

pub struct Delegate {}

impl Delegate {
    pub fn new() -> Self {
        Self {}
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
                match key_event.key {
                    Key::Escape => ctx.submit_command(QUIT_APP),
                    Key::ArrowLeft => {
                        data.previous();
                    }
                    Key::ArrowRight => {
                        data.next();
                    }
                    Key::Enter => {
                        if let Some(item) = data.get_selected_item() {
                            // TODO: don't print here but instead store/return the item
                            println!("{}", item);
                        }
                        ctx.submit_command(QUIT_APP);
                    }
                    Key::Tab => data.complete(),
                    Key::Backspace => data.delete_backward(),
                    Key::Character(c) => data.insert(&c),
                    _ => {}
                }
                None
            }
            _ => Some(event),
        }
    }
}
