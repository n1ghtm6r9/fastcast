use super::{capture_target, numbered_output};
use crate::capture::{NativeError, nvidia_output_for};
use desktop::OutputInfo;

#[test]
fn an_output_of_another_vendor_is_reported_with_its_adapter() {
    let outputs = [OutputInfo {
        vendor_id: 0x1002,
        adapter_name: "a test Radeon".to_owned(),
        ..numbered_output(0, 0, r"\\.\DISPLAY1")
    }];

    let failure =
        nvidia_output_for(&capture_target(), &outputs).expect_err("the adapter is not NVIDIA");

    match failure {
        NativeError::NotNvidia { adapter } => assert_eq!(adapter, "a test Radeon"),
        other => panic!("another failure: {other}"),
    }
}
