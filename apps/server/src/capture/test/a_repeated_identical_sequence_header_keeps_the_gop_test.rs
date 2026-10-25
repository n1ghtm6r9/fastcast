use super::{flv_stream, frame_tag, sequence_header_tag};
use crate::capture::pump_frames;
use crate::fixtures::nvenc_2560x1440_avcc;
use crate::hub::Hub;
use std::io::Cursor;

#[tokio::test]
async fn a_repeated_identical_sequence_header_keeps_the_gop() {
    let hub = Hub::new();
    let avcc = nvenc_2560x1440_avcc();
    let stream = flv_stream(&[
        sequence_header_tag(&avcc),
        frame_tag(0, true, &[0x65, 1]),
        sequence_header_tag(&avcc),
        frame_tag(16, false, &[0x41, 2]),
    ]);

    pump_frames(Cursor::new(stream), &hub, 60)
        .await
        .expect("the stream is valid");

    assert_eq!(hub.join().gop.len(), 2);
}
