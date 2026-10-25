use super::{avc_tag_data, demux_all, flv_header_bytes_declaring, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn a_header_longer_than_nine_bytes_is_skipped_whole() {
    let key_frame = [0, 0, 0, 2, 0x65, 3];
    let stream = [
        flv_header_bytes_declaring(13),
        flv_tag_bytes(9, 4, &avc_tag_data(1, 1, &key_frame)),
    ]
    .concat();

    assert_eq!(
        demux_all(stream).await,
        vec![FlvEvent::Frame {
            key: true,
            timestamp_ms: 4,
            data: Bytes::copy_from_slice(&key_frame),
        }]
    );
}
