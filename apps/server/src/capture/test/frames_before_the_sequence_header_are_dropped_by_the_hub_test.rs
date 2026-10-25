use super::{flv_stream, frame_tag, sequence_header_tag};
use crate::capture::pump_frames;
use crate::fixtures::nvenc_2560x1440_avcc;
use crate::hub::Hub;
use std::io::Cursor;

#[tokio::test]
async fn frames_before_the_sequence_header_are_dropped_by_the_hub() {
    let hub = Hub::new();
    let stream = flv_stream(&[
        frame_tag(0, true, &[0x65, 1]),
        sequence_header_tag(&nvenc_2560x1440_avcc()),
        frame_tag(16, true, &[0x65, 2]),
    ]);

    let report = pump_frames(Cursor::new(stream), &hub, 60)
        .await
        .expect("the stream is valid");
    let gop = hub.join().gop;

    assert_eq!(report.frames, 2);
    assert_eq!(gop.len(), 1);
    assert_eq!(gop[0].wire[1..5], 16u32.to_le_bytes());
}
