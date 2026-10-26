use super::{capture_target, default_settings, frame_while_running, hold_gpu_slot};
use crate::capture::{CaptureBackend, NativeBackend, NativePipeline};
use crate::hub::{EIGHT_BIT_DEPTH, Hub};
use media::VideoCodec;
use std::sync::Arc;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn a_cancelled_native_run_lets_the_next_one_capture() {
    hold_gpu_slot();
    let backend = NativeBackend::new(default_settings());
    let hub = Arc::new(Hub::new());
    let pipeline = NativePipeline {
        codec: VideoCodec::Hevc,
        bit_depth: EIGHT_BIT_DEPTH,
    };
    let target = capture_target();
    let mut subscription = hub.join();
    let mut first_run = Box::pin(backend.run(&hub, &pipeline, &target));

    frame_while_running(first_run.as_mut(), &mut subscription).await;
    drop(first_run);
    hub.reset();

    let mut subscription = hub.join();
    let mut second_run = Box::pin(backend.run(&hub, &pipeline, &target));
    let frame = frame_while_running(second_run.as_mut(), &mut subscription).await;

    assert!(frame.key);
}
