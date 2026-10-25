use crate::app::is_parsable_dotenv;

pub fn first_unparsable_line(text: &str) -> usize {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let parsable_lines = (1..=lines.len())
        .rev()
        .find(|&count| is_parsable_dotenv(&lines[..count].concat()))
        .unwrap_or(0);

    parsable_lines + 1
}
