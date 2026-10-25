use super::FakeFfmpeg;
use crate::capture::{FFMPEG_FILE_NAME, locate_ffmpeg};
use std::env;

#[test]
fn the_default_ffmpeg_comes_from_the_first_directory_that_has_it() {
    let missing = env::temp_dir().join("fastcast-no-such-directory");
    let first = FakeFfmpeg::installed();
    let second = FakeFfmpeg::installed();
    let search_dirs = [missing, first.directory.clone(), second.directory.clone()];

    assert_eq!(
        locate_ffmpeg("ffmpeg", &search_dirs),
        first.directory.join(FFMPEG_FILE_NAME).display().to_string()
    );
}
