use super::capture_target;
use crate::capture::{CaptureError, NativeError, run_native_capture};
use crate::hub::Hub;
use media::VideoCodec;
use std::sync::atomic::AtomicBool;

#[test]
fn native_capture_off_windows_fails_before_its_first_frame() {
    let failure = run_native_capture(
        &Hub::new(),
        VideoCodec::Avc,
        &capture_target(),
        1000,
        &AtomicBool::new(false),
    )
    .expect_err("native capture needs Windows");

    assert!(matches!(
        failure,
        CaptureError::Native {
            frames: 0,
            cause: NativeError::Unsupported
        }
    ));
}
