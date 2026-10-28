use super::an_hevc_key_frame;
use crate::capture::publish_encoded_frame;
use crate::hub::Hub;
use media::VideoCodec;
use std::time::Instant;

#[test]
fn an_encoded_frame_keeps_the_time_the_screen_showed_it() {
    let hub = Hub::new();
    let mut subscription = hub.join();
    let shown = Instant::now();

    publish_encoded_frame(
        &hub,
        VideoCodec::Hevc,
        60,
        true,
        &an_hevc_key_frame(),
        0,
        Some(shown),
    )
    .expect("the key frame describes the stream");

    let frame = subscription
        .frames
        .try_recv()
        .expect("the key frame is published");

    assert_eq!(frame.shown, Some(shown));
}
