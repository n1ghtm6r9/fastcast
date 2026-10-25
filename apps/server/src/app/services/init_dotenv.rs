use std::env;
use std::path::PathBuf;

pub fn init_dotenv() {
    let _ = dotenvy::dotenv();

    let executable_directory = env::current_exe()
        .ok()
        .and_then(|executable| executable.parent().map(PathBuf::from));

    if let Some(directory) = executable_directory {
        let _ = dotenvy::from_path(directory.join(".env"));
    }
}
