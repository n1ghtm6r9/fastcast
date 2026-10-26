use super::{
    capture_target, default_settings, frame_while_running, frames_within_while_running,
    hold_gpu_slot,
};
use crate::capture::{CaptureBackend, MIN_KEY_FRAME_INTERVAL, NativeBackend, NativePipeline};
use crate::hub::{EIGHT_BIT_DEPTH, Hub};
use media::VideoCodec;
use std::sync::Arc;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn a_key_frame_request_is_answered_with_an_avc_key_frame() {
    hold_gpu_slot();
    let backend = NativeBackend::new(default_settings());
    let hub = Arc::new(Hub::new());
    let pipeline = NativePipeline {
        codec: VideoCodec::Avc,
        bit_depth: EIGHT_BIT_DEPTH,
    };
    let target = capture_target();
    let mut subscription = hub.join();
    let mut run = Box::pin(backend.run(&hub, &pipeline, &target));

    let first = frame_while_running(run.as_mut(), &mut subscription).await;
    let second = frame_while_running(run.as_mut(), &mut subscription).await;

    assert!(first.key);
    assert!(
        !second.key,
        "the encoder keys by itself, so the request proves nothing"
    );

    hub.request_key_frame();

    let answered =
        frames_within_while_running(run.as_mut(), &mut subscription, MIN_KEY_FRAME_INTERVAL * 2)
            .await;

    assert!(
        answered
            .iter()
            .any(|frame| frame.key && frame.config.codec.starts_with("avc1."))
    );
}
