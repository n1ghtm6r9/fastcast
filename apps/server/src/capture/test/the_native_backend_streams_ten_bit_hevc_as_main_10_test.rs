use super::{capture_target, default_settings, frame_while_running, hold_gpu_slot};
use crate::capture::{CaptureBackend, NativeBackend, NativePipeline};
use crate::hub::{Hub, TEN_BIT_DEPTH};
use media::VideoCodec;
use std::sync::Arc;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn the_native_backend_streams_ten_bit_hevc_as_main_10() {
    hold_gpu_slot();
    let backend = NativeBackend::new(default_settings());
    let hub = Arc::new(Hub::new());
    let pipeline = NativePipeline {
        codec: VideoCodec::Hevc,
        bit_depth: TEN_BIT_DEPTH,
    };
    let target = capture_target();
    let mut subscription = hub.join();
    let mut run = Box::pin(backend.run(&hub, &pipeline, &target));

    let frame = frame_while_running(run.as_mut(), &mut subscription).await;

    assert!(frame.key);
    assert!(frame.config.codec.starts_with("hvc1.2."));
}
