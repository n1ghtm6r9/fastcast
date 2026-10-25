use super::{SAMPLE_HVCC, demux_all, enhanced_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::{FlvEvent, VideoCodec};
use bytes::Bytes;

#[tokio::test]
async fn an_enhanced_hevc_sequence_header_and_frames_come_out_in_order() {
    let key_frame = [0, 0, 0, 3, 0x26, 1, 2];
    let inter_frame = [0, 0, 0, 2, 0x02, 9];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &enhanced_tag_data(1, 0, b"hvc1", SAMPLE_HVCC)),
        flv_tag_bytes(9, 0, &enhanced_tag_data(1, 3, b"hvc1", &key_frame)),
        flv_tag_bytes(9, 16, &enhanced_tag_data(2, 3, b"hvc1", &inter_frame)),
    ]
    .concat();

    assert_eq!(
        demux_all(stream).await,
        vec![
            FlvEvent::SequenceHeader {
                codec: VideoCodec::Hevc,
                record: Bytes::from_static(SAMPLE_HVCC),
            },
            FlvEvent::Frame {
                key: true,
                timestamp_ms: 0,
                data: Bytes::copy_from_slice(&key_frame),
            },
            FlvEvent::Frame {
                key: false,
                timestamp_ms: 16,
                data: Bytes::copy_from_slice(&inter_frame),
            },
        ]
    );
}
