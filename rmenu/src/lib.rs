extern crate gl;

use exitfailure::ExitFailure;
use failure::ResultExt;
use glutin::PossiblyCurrent;

// use gl::types::*;

pub fn parse_color(mut color_string: &str) -> Result<[f32; 4], ExitFailure> {
    if color_string.starts_with('#') {
        color_string = color_string.get(1..).unwrap();
    }

    // TODO: can we avoid the extra String here and keep a &str pointer?
    let color_string_adjusted = match color_string.len() {
        3 => {
            let color_bytes = color_string.as_bytes();
            let [r, g, b] = [
                color_bytes[0] as char,
                color_bytes[1] as char,
                color_bytes[2] as char,
            ];
            format!("{0}{0}{1}{1}{2}{2}", r, g, b)
        }
        6 => color_string.to_owned(),
        x => {
            Err(failure::err_msg("failed to parse color")).with_context(
                |_| format!("string of length `{}` cannot parse as a color.", x)
            )?
        },
    };

    let mut bytes = [0u8; 3];
    hex::decode_to_slice(color_string_adjusted.as_str(), &mut bytes as &mut [u8])
        .with_context(|_| format!("failed to decode `{}` as a color.", color_string_adjusted))?;

    let color = [
        bytes[0] as f32 / 255.0,
        bytes[1] as f32 / 255.0,
        bytes[2] as f32 / 255.0,
        1.0,
    ];

    Ok(color)
}

pub struct Gl {}

pub fn load_gl(gl_context: &glutin::Context<PossiblyCurrent>) -> Gl {
    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| gl_context.get_proc_address(symbol) as *const _);

    Gl {}
}

impl Gl {
    pub fn clear(&self, color: [f32; 4]) {
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
