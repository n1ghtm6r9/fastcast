use crate::capture::{CaptureError, EncoderPicker};
use crate::settings::VideoEncoder;
use std::future::Future;
use tracing::{info, warn};

pub async fn pick_encoder<P, F>(
    picker: &EncoderPicker,
    probe: P,
) -> Result<VideoEncoder, CaptureError>
where
    P: Fn(VideoEncoder) -> F,
    F: Future<Output = Result<bool, CaptureError>>,
{
    if let Some(encoder) = picker.resolved() {
        return Ok(encoder);
    }

    while let Some(candidate) = picker.next_candidate() {
        if probe(candidate).await? {
            picker.remember(candidate);
            info!("video encoder chosen: {candidate}");

            return Ok(candidate);
        }

        warn!("video encoder {candidate} is not usable on this machine");
        picker.mark_failed(candidate);
    }

    let tried = picker.unusable();

    picker.forget_failures();

    Err(CaptureError::NoEncoderAvailable { tried })
}
