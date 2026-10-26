use super::{capture_target, default_settings, frame_while_running, hold_gpu_slot};
use crate::capture::{CaptureBackend, NativeBackend, NativePipeline};
use crate::hub::{EIGHT_BIT_DEPTH, Hub};
use media::VideoCodec;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn a_cancelled_native_run_publishes_nothing_once_it_has_returned() {
    hold_gpu_slot();
    let backend = NativeBackend::new(default_settings());
    let hub = Arc::new(Hub::new());
    let pipeline = NativePipeline {
        codec: VideoCodec::Hevc,
        bit_depth: EIGHT_BIT_DEPTH,
    };
    let target = capture_target();
    let mut subscription = hub.join();
    let mut run = Box::pin(backend.run(&hub, &pipeline, &target));

    frame_while_running(run.as_mut(), &mut subscription).await;
    hub.request_key_frame();
    drop(run);
    hub.reset();
    sleep(Duration::from_millis(200)).await;

    assert!(hub.join().config.is_none());
}
