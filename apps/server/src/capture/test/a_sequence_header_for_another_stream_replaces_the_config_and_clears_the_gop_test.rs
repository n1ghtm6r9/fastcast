use super::{flv_stream, frame_tag, sequence_header_tag};
use crate::capture::pump_frames;
use crate::fixtures::{nvenc_2560x1440_avcc, x264_1920x1080_avcc};
use crate::hub::Hub;
use std::io::Cursor;

#[tokio::test]
async fn a_sequence_header_for_another_stream_replaces_the_config_and_clears_the_gop() {
    let hub = Hub::new();
    let stream = flv_stream(&[
        sequence_header_tag(&nvenc_2560x1440_avcc()),
        frame_tag(0, true, &[0x65, 1]),
        sequence_header_tag(&x264_1920x1080_avcc()),
        frame_tag(16, false, &[0x41, 2]),
    ]);

    pump_frames(Cursor::new(stream), &hub, 60)
        .await
        .expect("the stream is valid");
    let subscription = hub.join();
    let config = subscription.config.expect("a config");

    assert_eq!((config.width, config.height), (1920, 1080));
    assert!(subscription.gop.is_empty());
}
