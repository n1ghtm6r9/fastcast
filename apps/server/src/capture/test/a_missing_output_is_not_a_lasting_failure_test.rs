use crate::capture::NativeError;

#[test]
fn a_missing_output_is_not_a_lasting_failure() {
    assert!(!NativeError::OutputMissing { index: 0 }.is_lasting());
}
