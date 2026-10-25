use super::FakeFfmpeg;
use crate::capture::{FFMPEG_FILE_NAME, locate_ffmpeg};
use std::slice;

#[test]
fn a_configured_ffmpeg_path_that_is_a_file_is_used_as_it_is() {
    let configured = FakeFfmpeg::installed();
    let searched = FakeFfmpeg::installed();
    let path = configured
        .directory
        .join(FFMPEG_FILE_NAME)
        .display()
        .to_string();

    assert_eq!(
        locate_ffmpeg(&path, slice::from_ref(&searched.directory)).expect("the file exists"),
        path
    );
}
