use super::FakeFfmpeg;
use crate::capture::locate_ffmpeg;
use std::env;

#[test]
fn the_default_ffmpeg_stays_a_bare_name_when_no_directory_has_it() {
    let missing = env::temp_dir().join("fastcast-no-such-directory");
    let script_only = FakeFfmpeg::exiting_at_once();

    assert_eq!(
        locate_ffmpeg("ffmpeg", &[missing, script_only.directory.clone()]),
        "ffmpeg"
    );
}
