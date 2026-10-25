use super::{
    SAMPLE_AVCC, SAMPLE_HVCC, avc_tag_data, demux_all, enhanced_tag_data, flv_header_bytes,
    flv_tag_bytes,
};
use crate::flv::{FlvEvent, VideoCodec};
use bytes::Bytes;

#[tokio::test]
async fn legacy_and_enhanced_video_tags_in_one_stream_are_both_demuxed() {
    let avc_key_frame = [0, 0, 0, 2, 0x65, 3];
    let hevc_key_frame = [0, 0, 0, 2, 0x26, 4];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 0, SAMPLE_AVCC)),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 1, &avc_key_frame)),
        flv_tag_bytes(9, 0, &enhanced_tag_data(1, 0, b"hvc1", SAMPLE_HVCC)),
        flv_tag_bytes(9, 5, &enhanced_tag_data(1, 3, b"hvc1", &hevc_key_frame)),
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
                data: Bytes::copy_from_slice(&avc_key_frame),
            },
            FlvEvent::SequenceHeader {
                codec: VideoCodec::Hevc,
                record: Bytes::from_static(SAMPLE_HVCC),
            },
            FlvEvent::Frame {
                key: true,
                timestamp_ms: 5,
                data: Bytes::copy_from_slice(&hevc_key_frame),
            },
        ]
    );
}
