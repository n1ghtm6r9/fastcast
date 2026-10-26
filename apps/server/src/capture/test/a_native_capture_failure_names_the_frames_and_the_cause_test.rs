use crate::capture::{CaptureError, NativeError};

#[test]
fn a_native_capture_failure_names_the_frames_and_the_cause() {
    let failure = CaptureError::Native {
        frames: 12,
        cause: NativeError::WorkerGone,
    };

    assert_eq!(
        failure.to_string(),
        "native capture failed after 12 frames: the capture thread stopped without a result"
    );
}
