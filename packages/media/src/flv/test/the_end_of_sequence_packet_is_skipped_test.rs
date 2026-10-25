use super::{avc_tag_data, demux_all, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn the_end_of_sequence_packet_is_skipped() {
    let key_frame = [0, 0, 0, 2, 0x65, 3];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 1, &key_frame)),
        flv_tag_bytes(9, 33, &avc_tag_data(1, 2, &[])),
    ]
    .concat();

    assert_eq!(
        demux_all(stream).await,
        vec![FlvEvent::Frame {
            key: true,
            timestamp_ms: 0,
            data: Bytes::copy_from_slice(&key_frame),
        }]
    );
}
