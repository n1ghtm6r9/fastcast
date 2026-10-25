use crate::app::{DOTENV_FILE_NAME, DotenvError, find_dotenv_file, load_dotenv_file};
use std::path::Path;

pub fn load_dotenv_files(
    current_directory: Option<&Path>,
    executable_directory: Option<&Path>,
) -> Result<(), DotenvError> {
    let nearest = current_directory.and_then(find_dotenv_file);
    let beside_executable = executable_directory.map(|directory| directory.join(DOTENV_FILE_NAME));

    for path in nearest.into_iter().chain(beside_executable) {
        load_dotenv_file(&path)?;
    }

    Ok(())
}
