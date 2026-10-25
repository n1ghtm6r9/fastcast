use super::{flv_stream, sequence_header_tag};
use crate::capture::pump_frames;
use crate::fixtures::nvenc_2560x1440_avcc;
use crate::hub::Hub;
use std::io::Cursor;

#[tokio::test]
async fn a_stream_with_no_frames_reports_zero_frames() {
    let header_only = flv_stream(&[]);
    let sequence_header_only = flv_stream(&[sequence_header_tag(&nvenc_2560x1440_avcc())]);

    let first = pump_frames(Cursor::new(header_only), &Hub::new(), 60).await;
    let second = pump_frames(Cursor::new(sequence_header_only), &Hub::new(), 60).await;

    assert_eq!(first.expect("a valid stream").frames, 0);
    assert_eq!(second.expect("a valid stream").frames, 0);
}
