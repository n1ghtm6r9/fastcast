use super::{SAMPLE_AVCC, avc_tag_data, demux_all, flv_header_bytes, flv_tag_bytes};
use crate::flv::{FlvEvent, VideoCodec};
use bytes::Bytes;

#[tokio::test]
async fn script_and_audio_tags_are_skipped() {
    let key_frame = [0, 0, 0, 2, 0x65, 7];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(18, 0, b"onMetaData and whatever follows it"),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 0, SAMPLE_AVCC)),
        flv_tag_bytes(8, 5, &[0xaf, 0x01, 1, 2, 3, 4]),
        flv_tag_bytes(9, 10, &avc_tag_data(1, 1, &key_frame)),
        flv_tag_bytes(8, 20, &[0xaf, 0x01]),
        flv_tag_bytes(18, 20, &[]),
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
                timestamp_ms: 10,
                data: Bytes::copy_from_slice(&key_frame),
            },
        ]
    );
}
