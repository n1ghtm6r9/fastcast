use super::{capture_target, numbered_output};
use crate::capture::nvidia_output_for;

#[test]
fn the_numbered_output_of_an_nvidia_adapter_is_taken() {
    let outputs = [numbered_output(0, 0, r"\\.\DISPLAY1")];

    let output = nvidia_output_for(&capture_target(), &outputs)
        .expect("the output is driven by an NVIDIA adapter");

    assert_eq!(output.device_name, r"\\.\DISPLAY1");
}
