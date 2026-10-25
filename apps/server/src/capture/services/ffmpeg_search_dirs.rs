use crate::capture::saved_path_dirs;
use std::env;
use std::path::PathBuf;

pub fn ffmpeg_search_dirs() -> Vec<PathBuf> {
    let executable_directory = env::current_exe()
        .ok()
        .and_then(|executable| executable.parent().map(PathBuf::from));
    let path = env::var_os("PATH");

    executable_directory
        .into_iter()
        .chain(path.iter().flat_map(env::split_paths))
        .chain(saved_path_dirs())
        .collect()
}
