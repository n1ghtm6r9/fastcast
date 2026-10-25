#[cfg(windows)]
use crate::capture::CREATE_NO_WINDOW;
use std::process::Stdio;
use tokio::process::Command;

pub fn ffmpeg_command(path: &str, args: &[String]) -> Command {
    let mut command = Command::new(path);

    command.args(args).stdin(Stdio::null()).kill_on_drop(true);

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    command
}
