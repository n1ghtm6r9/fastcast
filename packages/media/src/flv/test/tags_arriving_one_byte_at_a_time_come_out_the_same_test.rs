use super::{SAMPLE_AVCC, avc_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::{FlvDemuxer, FlvEvent, VideoCodec};
use bytes::Bytes;
use tokio::io::{AsyncWriteExt, duplex};

#[tokio::test]
async fn tags_arriving_one_byte_at_a_time_come_out_the_same() {
    let key_frame = [0, 0, 0, 5, 0x65, 1, 2, 3, 4];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(18, 0, b"metadata"),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 0, SAMPLE_AVCC)),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 1, &key_frame)),
    ]
    .concat();
    let (mut writer, reader) = duplex(1);

    let sender = tokio::spawn(async move {
        writer.write_all(&stream).await.unwrap();
    });

    let mut demuxer = FlvDemuxer::new(reader);

    assert_eq!(
        demuxer.next_event().await.unwrap(),
        Some(FlvEvent::SequenceHeader {
            codec: VideoCodec::Avc,
            record: Bytes::from_static(SAMPLE_AVCC),
        })
    );
    assert_eq!(
        demuxer.next_event().await.unwrap(),
        Some(FlvEvent::Frame {
            key: true,
            timestamp_ms: 0,
            data: Bytes::copy_from_slice(&key_frame),
        })
    );
    assert_eq!(demuxer.next_event().await.unwrap(), None);

    sender.await.unwrap();
}
