use crate::app::{DotenvError, decode_dotenv_text, unparsable_line_error, with_a_final_line_break};
use std::path::Path;
use std::{fs, io};

pub fn load_dotenv_file(path: &Path) -> Result<(), DotenvError> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(source) => {
            return Err(DotenvError::Read {
                path: path.to_path_buf(),
                source,
            });
        }
    };
    let text = decode_dotenv_text(&bytes)
        .map(with_a_final_line_break)
        .ok_or_else(|| DotenvError::NotText {
            path: path.to_path_buf(),
        })?;

    dotenvy::from_read(text.as_bytes()).map_err(|_| unparsable_line_error(path, &text))
}
