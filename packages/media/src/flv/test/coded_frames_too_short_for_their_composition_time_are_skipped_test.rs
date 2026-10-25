use super::{demux_all, enhanced_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn coded_frames_too_short_for_their_composition_time_are_skipped() {
    let key_frame = [0, 0, 0, 2, 0x26, 7];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &enhanced_tag_data(1, 1, b"hvc1", &[])),
        flv_tag_bytes(9, 0, &enhanced_tag_data(1, 1, b"hvc1", &[0, 0])),
        flv_tag_bytes(9, 2, &enhanced_tag_data(1, 3, b"hvc1", &key_frame)),
    ]
    .concat();

    assert_eq!(
        demux_all(stream).await,
        vec![FlvEvent::Frame {
            key: true,
            timestamp_ms: 2,
            data: Bytes::copy_from_slice(&key_frame),
        }]
    );
}
