use crate::capture::saved_path_dirs;
use std::env;
use std::path::PathBuf;

pub fn ffmpeg_search_dirs() -> Vec<PathBuf> {
    env::var_os("PATH")
        .iter()
        .flat_map(env::split_paths)
        .chain(saved_path_dirs())
        .collect()
}
