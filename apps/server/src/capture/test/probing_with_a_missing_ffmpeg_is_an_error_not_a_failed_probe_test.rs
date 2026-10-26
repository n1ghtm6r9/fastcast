use crate::capture::{CaptureError, VideoEncoder, probe_encoder};
use media::VideoCodec;

#[tokio::test]
async fn probing_with_a_missing_ffmpeg_is_an_error_not_a_failed_probe() {
    let outcome = probe_encoder(
        "fastcast-no-such-ffmpeg",
        VideoEncoder::Nvenc,
        VideoCodec::Avc,
    )
    .await;

    assert!(matches!(outcome, Err(CaptureError::FfmpegNotFound { .. })));
}
