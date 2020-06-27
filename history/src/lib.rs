use exitfailure::ExitFailure;
use failure::ResultExt;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::fmt::Write;

// TODO generalize from RandomState to generic
pub type HistoryItems = HashMap<String, i32, RandomState>;

pub fn parse_history_file(path: &std::path::PathBuf) -> Result<HistoryItems, ExitFailure> {
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("could not read file `{}`.", path.to_string_lossy()))?;

    let history_items_result: Result<HistoryItems, _> = content
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.splitn(2, ':').collect();

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
    history_items: HistoryItems,
) -> Result<(), ExitFailure> {
    let mut content = String::new();

    for (a, n) in history_items.iter() {
        writeln!(&mut content, "{}:{}", n, a)
            .with_context(|_| format!("could not format values `{}`, `{}`.", n, a))?;
    }

    Ok(std::fs::write(path, content)
        .with_context(|_| format!("failed to write content to `{}`.", path.to_string_lossy()))?)
}
