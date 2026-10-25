use crate::capture::{CaptureError, spawn_error_of};
use std::io::{Error, ErrorKind};

#[test]
fn a_missing_program_is_told_apart_from_other_spawn_failures() {
    let failure = spawn_error_of("ffmpeg", Error::from(ErrorKind::NotFound));

    assert!(matches!(failure, CaptureError::FfmpegNotFound { .. }));
}
