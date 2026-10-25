use super::{demux_all, enhanced_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn coded_frames_without_composition_time_keep_their_whole_payload() {
    let frame = [0, 0, 0, 3, 0x02, 1, 2];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 7, &enhanced_tag_data(2, 3, b"hvc1", &frame)),
    ]
    .concat();

    assert_eq!(
        demux_all(stream).await,
        vec![FlvEvent::Frame {
            key: false,
            timestamp_ms: 7,
            data: Bytes::copy_from_slice(&frame),
        }]
    );
}
