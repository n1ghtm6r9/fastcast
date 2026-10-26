use crate::capture::{
    CaptureError, CaptureReport, CaptureTarget, STDERR_FLUSH_TIMEOUT, VideoEncoder, ffmpeg_args,
    forward_stderr, pump_frames, spawn_ffmpeg, wait_for_exit,
};
use crate::hub::Hub;
use crate::settings::Settings;
use media::VideoCodec;
use tokio::time::timeout;
use tracing::warn;

pub async fn run_ffmpeg(
    ffmpeg_path: &str,
    settings: &Settings,
    target: &CaptureTarget,
    encoder: VideoEncoder,
    codec: VideoCodec,
    hub: &Hub,
) -> Result<CaptureReport, CaptureError> {
    let args = ffmpeg_args(settings, target, encoder, codec);
    let mut child = spawn_ffmpeg(ffmpeg_path, &args)?;
    let stdout = child.stdout.take().ok_or(CaptureError::MissingPipe)?;
    let stderr = child.stderr.take().ok_or(CaptureError::MissingPipe)?;
    let stderr_task = tokio::spawn(forward_stderr(stderr));

    let report = pump_frames(stdout, hub, target.fps).await;
    let status = wait_for_exit(&mut child).await;
    let _ = timeout(STDERR_FLUSH_TIMEOUT, stderr_task).await;

    if let Some(status) = status
        && !status.success()
    {
        warn!("ffmpeg exited with {status}");
    }

    report
}
