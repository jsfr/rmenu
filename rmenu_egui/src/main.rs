use cocoa::appkit::NSScreen;
use cocoa::base::nil;
use egui::{Pos2, Vec2};

fn get_main_screen_width() -> f32 {
    let frame = unsafe {
        let object = NSScreen::mainScreen(nil);
        object.frame()
    };

    let size = frame.size;

    size.width as f32
}

fn main() {
    let width = get_main_screen_width();
    let height = 30.0;

    let mut native_options = eframe::NativeOptions::default();
    native_options.decorated = false;
    native_options.initial_window_size = Some(Vec2::new(width, height));
    native_options.resizable = false;
    native_options.always_on_top = true;
    native_options.initial_window_pos = Some(Pos2::new(0.0, 0.0));

    eframe::run_native(
        "rmenu_egui",
        native_options,
        Box::new(|cc| Box::new(RmenuApp::new(cc))),
    );
}

#[derive(Default)]
struct RmenuApp {}

impl RmenuApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for RmenuApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
