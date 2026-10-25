use super::{SAMPLE_AVCC, avc_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::{FlvDemuxer, FlvEvent};
use bytes::Bytes;
use std::io::Cursor;

#[tokio::test]
async fn a_stream_that_ends_in_the_middle_of_a_tag_ends_with_none_after_the_tags_before_it() {
    let key_frame = [0, 0, 0, 40, 0x65, 1, 2, 3, 4, 5, 6, 7, 8];
    let whole_tag = flv_tag_bytes(9, 0, &avc_tag_data(1, 1, &key_frame));
    let cut_tag = &whole_tag[..whole_tag.len() / 2];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 0, SAMPLE_AVCC)),
        cut_tag.to_vec(),
    ]
    .concat();
    let mut demuxer = FlvDemuxer::new(Cursor::new(stream));

    assert_eq!(
        demuxer.next_event().await.unwrap(),
        Some(FlvEvent::SequenceHeader(Bytes::from_static(SAMPLE_AVCC)))
    );
    assert_eq!(demuxer.next_event().await.unwrap(), None);
}
