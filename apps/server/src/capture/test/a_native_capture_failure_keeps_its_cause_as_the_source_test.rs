use crate::capture::{CaptureError, NativeError};
use std::error::Error;

#[test]
fn a_native_capture_failure_keeps_its_cause_as_the_source() {
    let failure = CaptureError::Native {
        frames: 0,
        cause: NativeError::WorkerGone,
    };

    let source = failure.source().expect("the cause is the source");

    assert_eq!(
        source.to_string(),
        "the capture thread stopped without a result"
    );
}
