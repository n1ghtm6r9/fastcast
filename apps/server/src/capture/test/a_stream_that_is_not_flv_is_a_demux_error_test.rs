use crate::capture::{CaptureError, pump_frames};
use crate::hub::Hub;
use std::io::Cursor;

#[tokio::test]
async fn a_stream_that_is_not_flv_is_a_demux_error() {
    let stream = b"this is not a flv stream at all".to_vec();

    let outcome = pump_frames(Cursor::new(stream), &Hub::new(), 60).await;

    assert!(matches!(outcome, Err(CaptureError::Demux(_))));
}
