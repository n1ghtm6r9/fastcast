use super::{capture_target, default_settings, frame_while_running, hold_gpu_slot};
use crate::capture::{CaptureBackend, NativeBackend, NativePipeline};
use crate::hub::Hub;
use media::VideoCodec;
use std::sync::Arc;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn a_key_frame_request_is_answered_with_a_key_frame() {
    hold_gpu_slot();
    let backend = NativeBackend::new(default_settings());
    let hub = Arc::new(Hub::new());
    let pipeline = NativePipeline {
        codec: VideoCodec::Hevc,
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

    let mut answered = false;

    for _ in 0..5 {
        if frame_while_running(run.as_mut(), &mut subscription)
            .await
            .key
        {
            answered = true;
            break;
        }
    }

    assert!(answered);
}
