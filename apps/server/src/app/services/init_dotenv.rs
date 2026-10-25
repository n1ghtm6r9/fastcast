use crate::app::{DotenvError, load_dotenv_files};
use std::env;
use std::path::PathBuf;

pub fn init_dotenv() -> Result<(), DotenvError> {
    let current_directory = env::current_dir().ok();
    let executable_directory = env::current_exe()
        .ok()
        .and_then(|executable| executable.parent().map(PathBuf::from));

    load_dotenv_files(
        current_directory.as_deref(),
        executable_directory.as_deref(),
    )
}
