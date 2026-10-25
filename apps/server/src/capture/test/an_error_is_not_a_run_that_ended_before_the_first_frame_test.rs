use crate::capture::{CaptureError, ended_before_the_first_frame};

#[test]
fn an_error_is_not_a_run_that_ended_before_the_first_frame() {
    assert!(!ended_before_the_first_frame(&Err(
        CaptureError::MissingPipe
    )));
    assert!(!ended_before_the_first_frame(&Err(
        CaptureError::FfmpegNotFound {
            path: "ffmpeg".to_owned(),
        }
    )));
}
