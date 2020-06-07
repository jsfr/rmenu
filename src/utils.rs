use exitfailure::ExitFailure;
use failure::ResultExt;
use glutin::PossiblyCurrent;
use std::collections::HashMap;
use std::fmt::Write;

extern crate gl;
use gl::types::*;

pub fn parse_history_file(path: &std::path::PathBuf) -> Result<HashMap<String, i32>, ExitFailure> {
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("could not read file `{}`.", path.to_string_lossy()))?;

    let history_items_result: Result<HashMap<String, i32>, _> = content
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.splitn(2, ":").collect();

            match split_line.as_slice() {
                [n, a] => match n.parse::<i32>() {
                    Ok(parsed_n) => Ok((String::from(*a), parsed_n)),
                    _ => Err(failure::err_msg(format!(
                        "could not parse `{}` as an integer.",
                        n
                    ))),
                },
                _ => Err(failure::err_msg(format!("could not split `{}`.", line))),
            }
        })
        .collect();

    Ok(history_items_result.context(
        "could not parse history file.
                 each line should have the form `[number]:[item]`.
                 example: `1:Firefox`.",
    )?)
}

pub fn write_history_file(
    path: &std::path::PathBuf,
    history_items: HashMap<String, i32>,
) -> Result<(), ExitFailure> {
    let mut content = String::new();

    for (a, n) in history_items.iter() {
        writeln!(&mut content, "{}:{}", n, a)
            .with_context(|_| format!("could not format values `{}`, `{}`.", n, a))?;
    }

    Ok(std::fs::write(path, content)
        .with_context(|_| format!("failed to write content to `{}`.", path.to_string_lossy()))?)
}

pub fn parse_color(mut color_string: &str) -> Result<[f32; 4], ()> {
    if color_string.starts_with("#") {
        // TODO handle possible exception here better than .unwrap()
        color_string = color_string.get(1..).unwrap();
    }

    // TODO can we avoid the extra String here and keep a &str pointer?
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
        _ => return Err(()),
    };

    let mut bytes = [0u8; 3];
    hex::decode_to_slice(color_string_adjusted, &mut bytes as &mut [u8]);

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
