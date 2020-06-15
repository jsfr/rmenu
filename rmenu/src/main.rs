use exitfailure::ExitFailure;
use glutin::dpi::PhysicalPosition;
use glutin::dpi::PhysicalSize;
use glutin::event::VirtualKeyCode;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::platform::macos::{ActivationPolicy, WindowBuilderExtMacOS};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder};
use structopt::StructOpt;

use rmenu::{load_gl, parse_color};
use std::io;
use std::io::BufRead;
use regex::Regex;

/// GUI-based fuzzy selector for an arbitrary list of inputs
#[derive(StructOpt)]
#[structopt(name = "rmenu", version = "0.1.0")]
struct Cli {
    /// Defines the normal background color.
    /// accepted: #RGB, #RRGGBB
    /// example: --nb #112233
    #[structopt(long = "nb", default_value = "#000")]
    normal_background: String,

    /// Defines the normal foreground (text) color.
    /// accepted: #RGB, #RRGGBB
    /// example: --nf #112233
    #[structopt(long = "nf", default_value = "#FFF")]
    normal_foreground: String,

    /// Defines the selected background color.
    /// accepted: #RGB, #RRGGBB
    /// example: --sb #112233
    #[structopt(long = "sb", default_value = "#000")]
    selected_background: String,

    /// Defines the selected foreground (text) color.
    /// accepted: #RGB, #RRGGBB
    /// example: --sf #112233
    #[structopt(long = "sf", default_value = "#FFF")]
    selected_foreground: String,
}

// TODO: Make as many fields as possible into references for improved performance
struct Selector {
    selection_text: String,
    selected_item: Option<String>,
    items: Vec<String>,
    visible_items: Vec<String>,
}

impl Selector {
    pub fn new(items: Vec<String>) -> Selector {
        Selector {
            selection_text: String::new(),
            selected_item: items.to_owned().get(0).map(|i| (*i).to_owned()),
            items: items.to_owned(),
            visible_items: items,
        }
    }

    pub fn push(&mut self, ch: char) {
        self.selection_text.push(ch);
        self.update_selection();
    }

    pub fn pop(&mut self) {
        self.selection_text.pop();
        self.update_selection();
    }

    fn update_selection(&mut self) {
        let pattern = format!(r"^.*{}.*$", self.selection_text);
        let re = Regex::new(pattern.as_str()).unwrap();
        self.visible_items = self.items.to_owned().into_iter().filter(|item| re.is_match(item)).collect();
        self.selected_item = self.visible_items.get(0).map(|i| (*i).to_owned());
    }
}

// TODO: fix all unwraps to provide ExitFailure's instead
fn main() -> Result<(), ExitFailure> {
    let args: Cli = Cli::from_args();
    let event_loop = EventLoop::new();

    let monitor = event_loop.primary_monitor();
    let width = monitor.size().width;
    let height = 30_f64 * monitor.scale_factor();

    let window_builder = WindowBuilder::new()
        .with_decorations(false)
        .with_always_on_top(true)
        .with_resizable(false)
        .with_activation_policy(ActivationPolicy::Accessory)
        .with_inner_size(PhysicalSize::new(width, height as u32))
        .with_visible(false);

    let windowed_context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { windowed_context.make_current().unwrap() };

    // Load the gl context to render things
    let gl = load_gl(&gl_window);

    // Position the window at the top of the screen, set the background color and make it visible
    gl.clear(parse_color(args.normal_background.as_str()).unwrap());
    gl_window
        .window()
        .set_outer_position(PhysicalPosition::new(0, 0));
    gl_window.window().set_visible(true);

    let items = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap();
    let mut selector = Selector::new(items);

    event_loop.run(move |event, _, control_flow| {
        // debug call to check out the events
        // println!("{:?}", event);

        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => {},
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic,
                } => {
                    if is_synthetic {
                        return
                    }

                    if let Some(code) = input.virtual_keycode {
                        match code {
                            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                            VirtualKeyCode::Back => selector.pop(),
                            VirtualKeyCode::Return => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    }
                }
                WindowEvent::ReceivedCharacter(c) => selector.push(c),
                // WindowEvent::Focused(false) |
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {},
            },
            Event::RedrawRequested(_) => {
                gl_window.swap_buffers().unwrap();
            }
            _ => {},
        }
    });
}
