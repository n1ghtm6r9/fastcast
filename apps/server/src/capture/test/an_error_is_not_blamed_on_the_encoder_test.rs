use crate::capture::{CaptureError, encoder_failed};

#[test]
fn an_error_is_not_blamed_on_the_encoder() {
    assert!(!encoder_failed(&Err(CaptureError::MissingPipe)));
    assert!(!encoder_failed(&Err(CaptureError::FfmpegNotFound {
        path: "ffmpeg".to_owned(),
    })));
}
