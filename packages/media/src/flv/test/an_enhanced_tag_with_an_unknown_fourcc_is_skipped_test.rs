use super::{demux_all, enhanced_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn an_enhanced_tag_with_an_unknown_fourcc_is_skipped() {
    let key_frame = [0, 0, 0, 2, 0x26, 7];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &enhanced_tag_data(1, 0, b"av01", &[1, 2, 3])),
        flv_tag_bytes(9, 0, &enhanced_tag_data(1, 3, b"vp09", &[4, 5, 6])),
        flv_tag_bytes(9, 8, &enhanced_tag_data(1, 3, b"hvc1", &key_frame)),
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
