use exitfailure::ExitFailure;
use glutin::dpi::PhysicalPosition;
use glutin::dpi::PhysicalSize;
use glutin::event::VirtualKeyCode;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::platform::macos::{ActivationPolicy, WindowBuilderExtMacOS};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, PossiblyCurrent};
use structopt::StructOpt;

use utils::{load_gl, parse_color};

/// GUI-based fuzzy selector for an arbitrary list of inputs
#[derive(StructOpt)]
#[structopt(name = "rmenu", version = "0.1.0")]
struct Cli {
    /// Defines the normal background color.
    /// accepted: #RGB, #RRGGBB
    /// example: --nb #112233
    #[structopt(long = "nb", default_value = "#000")]
    normal_background: String,
}

fn main() {
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

    event_loop.run(move |event, _, control_flow| {
        // debug call to check out the events
        println!("{:?}", event);

        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    if !is_synthetic {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    }
                }
                // WindowEvent::Focused(false) |
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                gl_window.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
