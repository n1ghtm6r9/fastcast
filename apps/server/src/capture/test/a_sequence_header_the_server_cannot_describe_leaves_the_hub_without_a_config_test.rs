use super::{flv_stream, frame_tag, sequence_header_tag};
use crate::capture::pump_frames;
use crate::hub::Hub;
use std::io::Cursor;

#[tokio::test]
async fn a_sequence_header_the_server_cannot_describe_leaves_the_hub_without_a_config() {
    let hub = Hub::new();
    let undescribable = [
        0x01, 0x64, 0x00, 0x2a, 0xff, 0xe1, 0x00, 0x04, 0x67, 0x64, 0x00, 0x2a, 0x01, 0x00, 0x03,
        0x68, 0xee, 0x3c,
    ];
    let stream = flv_stream(&[
        sequence_header_tag(&undescribable),
        frame_tag(0, true, &[0x65, 1]),
    ]);

    pump_frames(Cursor::new(stream), &hub, 60)
        .await
        .expect("the stream is valid");
    let subscription = hub.join();

    assert!(subscription.config.is_none());
    assert!(subscription.gop.is_empty());
}
