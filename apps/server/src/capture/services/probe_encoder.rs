use crate::capture::{
    CaptureError, PROBE_TIMEOUT, VideoEncoder, codec_name_of, ffmpeg_command, probe_args,
    spawn_error_of,
};
use media::VideoCodec;
use std::process::Stdio;
use tokio::time::timeout;
use tracing::debug;

pub async fn probe_encoder(
    ffmpeg_path: &str,
    encoder: VideoEncoder,
    codec: VideoCodec,
) -> Result<bool, CaptureError> {
    let name = codec_name_of(encoder, codec);
    let child = ffmpeg_command(ffmpeg_path, &probe_args(encoder, codec))
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| spawn_error_of(ffmpeg_path, source))?;

    match timeout(PROBE_TIMEOUT, child.wait_with_output()).await {
        Ok(Ok(output)) if output.status.success() => Ok(true),
        Ok(Ok(output)) => {
            let reason = String::from_utf8_lossy(&output.stderr);

            debug!("the {name} probe failed: {}", reason.trim());

            Ok(false)
        }
        Ok(Err(error)) => {
            debug!("the {name} probe could not be awaited: {error}");

            Ok(false)
        }
        Err(_) => {
            debug!("the {name} probe took longer than {PROBE_TIMEOUT:?}");

            Ok(false)
        }
    }
}
