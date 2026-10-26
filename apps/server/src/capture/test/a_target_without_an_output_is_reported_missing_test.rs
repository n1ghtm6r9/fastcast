use super::{capture_target, numbered_output};
use crate::capture::{CaptureTarget, NativeError, nvidia_output_for};

#[test]
fn a_target_without_an_output_is_reported_missing() {
    let outputs = [numbered_output(0, 0, r"\\.\DISPLAY1")];
    let target = CaptureTarget {
        output: 3,
        ..capture_target()
    };

    let failure = nvidia_output_for(&target, &outputs).expect_err("output 3 is not there");

    assert!(matches!(failure, NativeError::OutputMissing { index: 3 }));
}
