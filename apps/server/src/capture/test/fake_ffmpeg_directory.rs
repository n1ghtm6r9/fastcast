use super::next_fake_ffmpeg_id;
use std::path::PathBuf;
use std::{env, fs, process};

pub fn fake_ffmpeg_directory() -> PathBuf {
    let directory = env::temp_dir().join(format!(
        "fastcast-fake-ffmpeg-{}-{}",
        process::id(),
        next_fake_ffmpeg_id()
    ));

    fs::create_dir_all(&directory).expect("the temporary directory is created");

    directory
}
