use crate::capture::{CaptureError, ffmpeg_command, spawn_error_of};
use std::process::Stdio;
use tokio::process::Child;

pub fn spawn_ffmpeg(path: &str, args: &[String]) -> Result<Child, CaptureError> {
    ffmpeg_command(path, args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| spawn_error_of(path, source))
}
