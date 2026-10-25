use crate::capture::{CaptureError, spawn_error_of};
use std::io::{Error, ErrorKind};

#[test]
fn a_spawn_failure_other_than_a_missing_program_keeps_its_cause() {
    let failure = spawn_error_of("ffmpeg", Error::from(ErrorKind::PermissionDenied));

    assert!(matches!(
        &failure,
        CaptureError::Spawn { path, source }
            if path == "ffmpeg" && source.kind() == ErrorKind::PermissionDenied
    ));
    assert!(
        failure
            .to_string()
            .starts_with("could not start \"ffmpeg\"")
    );
}
