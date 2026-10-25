use super::{EXITS_AT_ONCE_SCRIPT, FakeFfmpeg, fake_ffmpeg_directory};
use std::fs;

impl FakeFfmpeg {
    pub fn exiting_at_once() -> Self {
        let directory = fake_ffmpeg_directory();

        fs::write(directory.join("ffmpeg.cmd"), EXITS_AT_ONCE_SCRIPT)
            .expect("the script is written");

        Self { directory }
    }
}
