use super::{capture_target, numbered_output};
use crate::capture::native_output_for;

#[test]
fn an_output_of_another_adapter_is_not_the_numbered_output() {
    let outputs = [numbered_output(1, 0, r"\\.\DISPLAY3")];

    assert!(native_output_for(&capture_target(), &outputs).is_none());
}
