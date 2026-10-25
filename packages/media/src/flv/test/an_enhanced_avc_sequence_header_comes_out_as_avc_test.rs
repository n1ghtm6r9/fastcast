use super::{SAMPLE_AVCC, demux_all, enhanced_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::{FlvEvent, VideoCodec};
use bytes::Bytes;

#[tokio::test]
async fn an_enhanced_avc_sequence_header_comes_out_as_avc() {
    let key_frame = [0, 0, 0, 3, 0x65, 1, 2];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &enhanced_tag_data(1, 0, b"avc1", SAMPLE_AVCC)),
        flv_tag_bytes(9, 4, &enhanced_tag_data(1, 3, b"avc1", &key_frame)),
    ]
    .concat();

    assert_eq!(
        demux_all(stream).await,
        vec![
            FlvEvent::SequenceHeader {
                codec: VideoCodec::Avc,
                record: Bytes::from_static(SAMPLE_AVCC),
            },
            FlvEvent::Frame {
                key: true,
                timestamp_ms: 4,
                data: Bytes::copy_from_slice(&key_frame),
            },
        ]
    );
}
