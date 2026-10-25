use super::{EXITS_AT_ONCE_SCRIPT, FakeFfmpeg, next_fake_ffmpeg_id};
use std::{env, fs, process};

impl FakeFfmpeg {
    pub fn exiting_at_once() -> Self {
        let directory = env::temp_dir().join(format!(
            "fastcast-fake-ffmpeg-{}-{}",
            process::id(),
            next_fake_ffmpeg_id()
        ));

        fs::create_dir_all(&directory).expect("the temporary directory is created");
        fs::write(directory.join("ffmpeg.cmd"), EXITS_AT_ONCE_SCRIPT)
            .expect("the script is written");

        Self { directory }
    }
}
