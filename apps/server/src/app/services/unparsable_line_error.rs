use crate::app::{DotenvError, first_unparsable_line};
use std::path::Path;

pub fn unparsable_line_error(path: &Path, text: &str) -> DotenvError {
    let number = first_unparsable_line(text);

    DotenvError::Line {
        path: path.to_path_buf(),
        number,
        text: text.lines().nth(number - 1).unwrap_or_default().to_owned(),
    }
}
