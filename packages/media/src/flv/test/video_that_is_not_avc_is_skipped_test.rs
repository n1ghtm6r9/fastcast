use super::{avc_tag_data, demux_all, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn video_that_is_not_avc_is_skipped() {
    let sorenson_h263_key_frame = [(1 << 4) | 2, 0, 0, 0, 0, 9, 9, 9];
    let key_frame = [0, 0, 0, 2, 0x65, 3];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &sorenson_h263_key_frame),
        flv_tag_bytes(9, 8, &avc_tag_data(1, 1, &key_frame)),
    ]
    .concat();

    assert_eq!(
        demux_all(stream).await,
        vec![FlvEvent::Frame {
            key: true,
            timestamp_ms: 8,
            data: Bytes::copy_from_slice(&key_frame),
        }]
    );
}
