use super::{capture_target, numbered_output};
use crate::capture::{CaptureTarget, native_output_for};

#[test]
fn the_native_output_is_the_numbered_output_of_the_first_adapter() {
    let outputs = [
        numbered_output(0, 0, r"\\.\DISPLAY1"),
        numbered_output(0, 1, r"\\.\DISPLAY2"),
        numbered_output(1, 0, r"\\.\DISPLAY3"),
    ];
    let target = CaptureTarget {
        output: 1,
        ..capture_target()
    };

    let output = native_output_for(&target, &outputs).expect("the first adapter has output 1");

    assert_eq!(output.device_name, r"\\.\DISPLAY2");
}
