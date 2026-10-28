use super::{
    capture_target, default_settings, frame_while_running, frames_within_while_running,
    hold_gpu_slot,
};
use crate::capture::{CaptureBackend, NativeBackend, NativePipeline};
use crate::hub::{EIGHT_BIT_DEPTH, Hub};
use media::VideoCodec;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn forced_key_frames_come_no_closer_than_the_minimum_interval() {
    hold_gpu_slot();
    let backend = NativeBackend::new(default_settings());
    let hub = Arc::new(Hub::new());
    let pipeline = NativePipeline {
        codec: VideoCodec::Hevc,
        bit_depth: EIGHT_BIT_DEPTH,
        noise: false,
    };
    let target = capture_target();
    let mut subscription = hub.join();
    let mut run = Box::pin(backend.run(&hub, &pipeline, &target));

    frame_while_running(run.as_mut(), &mut subscription).await;

    let asking = tokio::spawn({
        let hub = hub.clone();

        async move {
            loop {
                hub.request_key_frame();
                sleep(Duration::from_millis(2)).await;
            }
        }
    });

    let frames =
        frames_within_while_running(run.as_mut(), &mut subscription, Duration::from_secs(1)).await;

    asking.abort();

    let key_frames = frames.iter().filter(|frame| frame.key).count();

    println!(
        "key frames in the second after the first, asked for all the time: {key_frames} of {}",
        frames.len()
    );

    assert!(
        (2..=5).contains(&key_frames),
        "{key_frames} key frames in one second while one was asked for all the time"
    );
}
