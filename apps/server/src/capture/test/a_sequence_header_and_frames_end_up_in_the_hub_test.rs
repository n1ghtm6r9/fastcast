use super::{flv_stream, frame_tag, sequence_header_tag};
use crate::capture::pump_frames;
use crate::fixtures::nvenc_2560x1440_avcc;
use crate::hub::Hub;
use std::io::Cursor;

#[tokio::test]
async fn a_sequence_header_and_frames_end_up_in_the_hub() {
    let hub = Hub::new();
    let stream = flv_stream(&[
        sequence_header_tag(&nvenc_2560x1440_avcc()),
        frame_tag(0, true, &[0x65, 1]),
        frame_tag(16, false, &[0x41, 2]),
    ]);

    let report = pump_frames(Cursor::new(stream), &hub, 60)
        .await
        .expect("the stream is valid");
    let subscription = hub.join();
    let config = subscription
        .config
        .expect("the sequence header made a config");

    assert_eq!(report.frames, 2);
    assert_eq!((config.width, config.height, config.fps), (2560, 1440, 60));
    assert_eq!(subscription.gop.len(), 2);
    assert!(subscription.gop[0].key);
}
