use super::{SAMPLE_AVCC, avc_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvDemuxer;
use std::io::Cursor;

#[tokio::test]
async fn a_stream_that_ends_between_tags_ends_with_none_and_stays_ended() {
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, 0, &avc_tag_data(1, 0, SAMPLE_AVCC)),
    ]
    .concat();
    let mut demuxer = FlvDemuxer::new(Cursor::new(stream));

    assert!(demuxer.next_event().await.unwrap().is_some());
    assert!(demuxer.next_event().await.unwrap().is_none());
    assert!(demuxer.next_event().await.unwrap().is_none());
}
