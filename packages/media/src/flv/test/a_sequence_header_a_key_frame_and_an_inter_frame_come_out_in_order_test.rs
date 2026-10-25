use super::{SAMPLE_AVCC, avc_tag_data, demux_all, flv_header_bytes, flv_tag_bytes};
use crate::flv::{FlvEvent, VideoCodec};
use bytes::Bytes;

#[tokio::test]
async fn a_sequence_header_a_key_frame_and_an_inter_frame_come_out_in_order() {
    let key_frame = [0, 0, 0, 3, 0x65, 1, 2];
    let inter_frame = [0, 0, 0, 2, 0x41, 9];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 0, SAMPLE_AVCC)),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 1, &key_frame)),
        flv_tag_bytes(9, 16, &avc_tag_data(2, 1, &inter_frame)),
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
