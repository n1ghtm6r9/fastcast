use super::FakeFfmpeg;
use crate::capture::locate_ffmpeg;
use std::slice;

#[test]
fn a_configured_ffmpeg_path_is_kept_as_it_is() {
    let installed = FakeFfmpeg::installed();

    assert_eq!(
        locate_ffmpeg(
            "D:\\tools\\ffmpeg.exe",
            slice::from_ref(&installed.directory)
        ),
        "D:\\tools\\ffmpeg.exe"
    );
}
