use crate::capture::{CaptureError, PROBE_TIMEOUT, ffmpeg_command, probe_args, spawn_error_of};
use crate::settings::VideoEncoder;
use std::process::Stdio;
use tokio::time::timeout;
use tracing::debug;

pub async fn probe_encoder(ffmpeg_path: &str, encoder: VideoEncoder) -> Result<bool, CaptureError> {
    let child = ffmpeg_command(ffmpeg_path, &probe_args(encoder))
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| spawn_error_of(ffmpeg_path, source))?;

    match timeout(PROBE_TIMEOUT, child.wait_with_output()).await {
        Ok(Ok(output)) if output.status.success() => Ok(true),
        Ok(Ok(output)) => {
            let reason = String::from_utf8_lossy(&output.stderr);

            debug!("the {encoder} probe failed: {}", reason.trim());

            Ok(false)
        }
        Ok(Err(error)) => {
            debug!("the {encoder} probe could not be awaited: {error}");

            Ok(false)
        }
        Err(_) => {
            debug!("the {encoder} probe took longer than {PROBE_TIMEOUT:?}");

            Ok(false)
        }
    }
}
