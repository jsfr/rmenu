use std::sync::{Arc, Mutex};

use crate::ui_data::AppData;
use druid::{
    commands::QUIT_APP, keyboard_types::Key, AppDelegate, DelegateCtx, Env, Event, WindowId,
};

pub struct Delegate {
    pub result: Arc<Mutex<Option<String>>>,
}

impl Delegate {
    pub fn new(result: Arc<Mutex<Option<String>>>) -> Self {
        Self { result }
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
                        *self.result.lock().unwrap() =
                            data.get_selected_item().map(|i| i.value.to_string());
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
