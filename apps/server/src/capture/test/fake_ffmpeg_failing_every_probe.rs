use super::{FAILS_EVERY_PROBE_SCRIPT, FakeFfmpeg, fake_ffmpeg_directory};
use std::fs;

impl FakeFfmpeg {
    pub fn failing_every_probe() -> Self {
        let directory = fake_ffmpeg_directory();

        fs::write(directory.join("ffmpeg.cmd"), FAILS_EVERY_PROBE_SCRIPT)
            .expect("the script is written");

        Self { directory }
    }
}
