use crate::capture::ffmpeg_search_dirs;
use std::env;
use std::path::PathBuf;

#[test]
fn the_folder_of_the_server_executable_is_searched_for_ffmpeg_first() {
    let executable = env::current_exe().expect("the test knows its own executable");

    assert_eq!(
        ffmpeg_search_dirs().first().map(PathBuf::as_path),
        executable.parent()
    );
}
