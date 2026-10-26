use super::{default_settings, frame_while_running, frames_within_while_running, hold_gpu_slot};
use crate::capture::{CaptureBackend, CaptureTarget, NativeBackend, NativePipeline};
use crate::hub::{EIGHT_BIT_DEPTH, Hub};
use media::VideoCodec;
use std::sync::Arc;
use std::time::Duration;

pub async fn frames_in_the_second_after_the_first(fps: u32) -> usize {
    hold_gpu_slot();
    let backend = NativeBackend::new(default_settings());
    let hub = Arc::new(Hub::new());
    let pipeline = NativePipeline {
        codec: VideoCodec::Hevc,
        bit_depth: EIGHT_BIT_DEPTH,
    };
    let target = CaptureTarget { output: 0, fps };
    let mut subscription = hub.join();
    let mut run = Box::pin(backend.run(&hub, &pipeline, &target));

    frame_while_running(run.as_mut(), &mut subscription).await;

    let frames =
        frames_within_while_running(run.as_mut(), &mut subscription, Duration::from_secs(1))
            .await
            .len();

    println!("frames in the second after the first, at {fps} fps: {frames}");

    frames
}
