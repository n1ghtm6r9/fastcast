use super::{avc_tag_data, demux_all, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn a_video_tag_too_short_to_carry_an_avc_header_is_skipped() {
    let key_frame = [0, 0, 0, 2, 0x65, 3];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &[0x17, 0x01, 0x00]),
        flv_tag_bytes(9, 0, &[]),
        flv_tag_bytes(9, 2, &avc_tag_data(1, 1, &key_frame)),
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
