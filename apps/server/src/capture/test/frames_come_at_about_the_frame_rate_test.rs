use super::{
    capture_target, default_settings, frame_while_running, frames_within_while_running,
    hold_gpu_slot,
};
use crate::capture::{CaptureBackend, NativeBackend, NativePipeline};
use crate::hub::Hub;
use media::VideoCodec;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn frames_come_at_about_the_frame_rate() {
    hold_gpu_slot();
    let backend = NativeBackend::new(default_settings());
    let hub = Arc::new(Hub::new());
    let pipeline = NativePipeline {
        codec: VideoCodec::Hevc,
    };
    let target = capture_target();
    let mut subscription = hub.join();
    let mut run = Box::pin(backend.run(&hub, &pipeline, &target));

    frame_while_running(run.as_mut(), &mut subscription).await;
    let frames =
        frames_within_while_running(run.as_mut(), &mut subscription, Duration::from_secs(1))
            .await
            .len();

    println!(
        "frames in the second after the first, at {} fps: {frames}",
        target.fps
    );

    assert!(
        (50..=62).contains(&frames),
        "{frames} frames in one second at {} fps",
        target.fps
    );
}
