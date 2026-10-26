use super::{capture_target, default_settings, frame_while_running, hold_gpu_slot};
use crate::capture::{CaptureBackend, NativeBackend, NativePipeline};
use crate::hub::{EIGHT_BIT_DEPTH, Hub};
use media::VideoCodec;
use std::sync::Arc;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn the_native_backend_starts_with_a_described_key_frame() {
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

    let frame = frame_while_running(run.as_mut(), &mut subscription).await;

    assert!(frame.key);
    assert!(frame.config.codec.starts_with("hvc1."));
    assert!(frame.config.width > 0);
    assert!(frame.config.height > 0);
}
