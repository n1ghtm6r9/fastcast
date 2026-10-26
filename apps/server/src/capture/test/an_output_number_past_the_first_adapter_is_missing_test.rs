use super::{capture_target, numbered_output};
use crate::capture::{CaptureTarget, native_output_for};

#[test]
fn an_output_number_past_the_first_adapter_is_missing() {
    let outputs = [
        numbered_output(0, 0, r"\\.\DISPLAY1"),
        numbered_output(0, 1, r"\\.\DISPLAY2"),
        numbered_output(1, 0, r"\\.\DISPLAY3"),
    ];
    let target = CaptureTarget {
        output: 2,
        ..capture_target()
    };

    assert!(native_output_for(&target, &outputs).is_none());
}
