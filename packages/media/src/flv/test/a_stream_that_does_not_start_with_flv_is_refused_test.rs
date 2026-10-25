use crate::flv::{FlvDemuxer, FlvError};
use std::io::Cursor;

#[tokio::test]
async fn a_stream_that_does_not_start_with_flv_is_refused() {
    let mut demuxer = FlvDemuxer::new(Cursor::new(b"RIFF....WAVEfmt ".to_vec()));

    let refusal = demuxer.next_event().await.unwrap_err();

    assert!(
        matches!(refusal, FlvError::InvalidHeader),
        "got {:?}",
        refusal
    );
}

#[tokio::test]
async fn a_header_that_declares_a_length_shorter_than_itself_is_refused() {
    let mut header = b"FLV".to_vec();
    header.extend_from_slice(&[1, 1, 0, 0, 0, 4, 0, 0, 0, 0]);
    let mut demuxer = FlvDemuxer::new(Cursor::new(header));

    let refusal = demuxer.next_event().await.unwrap_err();

    assert!(
        matches!(refusal, FlvError::InvalidHeader),
        "got {:?}",
        refusal
    );
}
