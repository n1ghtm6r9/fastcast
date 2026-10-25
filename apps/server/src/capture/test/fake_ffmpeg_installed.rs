use super::{FakeFfmpeg, fake_ffmpeg_directory};
use crate::capture::FFMPEG_FILE_NAME;
use std::fs;

impl FakeFfmpeg {
    pub fn installed() -> Self {
        let directory = fake_ffmpeg_directory();

        fs::write(directory.join(FFMPEG_FILE_NAME), b"").expect("the program file is written");

        Self { directory }
    }
}
