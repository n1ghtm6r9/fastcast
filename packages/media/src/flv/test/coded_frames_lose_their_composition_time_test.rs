use super::{demux_all, enhanced_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn coded_frames_lose_their_composition_time() {
    let frame = [0, 0, 0, 3, 0x02, 1, 2];
    let after_a_positive_offset = [&[0x00, 0x00, 0x21][..], &frame].concat();
    let after_a_negative_offset = [&[0xff, 0xff, 0xf0][..], &frame].concat();
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(
            9,
            100,
            &enhanced_tag_data(2, 1, b"hvc1", &after_a_positive_offset),
        ),
        flv_tag_bytes(
            9,
            116,
            &enhanced_tag_data(2, 1, b"hvc1", &after_a_negative_offset),
        ),
    ]
    .concat();

    assert_eq!(
        demux_all(stream).await,
        vec![
            FlvEvent::Frame {
                key: false,
                timestamp_ms: 100,
                data: Bytes::copy_from_slice(&frame),
            },
            FlvEvent::Frame {
                key: false,
                timestamp_ms: 116,
                data: Bytes::copy_from_slice(&frame),
            },
        ]
    );
}
