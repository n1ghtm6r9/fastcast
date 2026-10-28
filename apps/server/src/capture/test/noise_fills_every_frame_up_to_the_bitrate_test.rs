use super::{default_settings, frame_while_running, frames_within_while_running, hold_gpu_slot};
use crate::capture::{CaptureBackend, CaptureTarget, NativeBackend, NativePipeline};
use crate::hub::{EIGHT_BIT_DEPTH, Hub};
use media::VideoCodec;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn noise_fills_every_frame_up_to_the_bitrate() {
    hold_gpu_slot();
    let mut settings = default_settings();
    settings.video_bitrate_kbps = 200_000;
    let frame_budget = settings.video_bitrate_kbps as usize * 1000 / 8 / 60;
    let backend = NativeBackend::new(settings);
    let hub = Arc::new(Hub::new());
    let pipeline = NativePipeline {
        codec: VideoCodec::Hevc,
        bit_depth: EIGHT_BIT_DEPTH,
        noise: true,
    };
    let target = CaptureTarget { output: 0, fps: 60 };
    let mut subscription = hub.join();
    let mut run = Box::pin(backend.run(&hub, &pipeline, &target));

    frame_while_running(run.as_mut(), &mut subscription).await;
    let frames =
        frames_within_while_running(run.as_mut(), &mut subscription, Duration::from_millis(500))
            .await;
    let sizes: Vec<usize> = frames.iter().map(|frame| frame.wire.len()).collect();

    assert!(sizes.len() > 20, "{sizes:?}");
    assert!(
        sizes
            .iter()
            .all(|size| size.abs_diff(frame_budget) < frame_budget / 10),
        "budget {frame_budget}: {sizes:?}"
    );
}
