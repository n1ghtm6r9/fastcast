use super::flv_header_bytes;
use crate::flv::FlvDemuxer;
use std::io::Cursor;

#[tokio::test]
async fn an_empty_stream_has_no_events() {
    let mut demuxer = FlvDemuxer::new(Cursor::new(Vec::<u8>::new()));

    assert_eq!(demuxer.next_event().await.unwrap(), None);
}

#[tokio::test]
async fn a_stream_with_only_a_header_has_no_events() {
    let mut demuxer = FlvDemuxer::new(Cursor::new(flv_header_bytes()));

    assert_eq!(demuxer.next_event().await.unwrap(), None);
}
