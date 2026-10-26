use crate::capture::NativeError;

#[test]
fn native_capture_off_windows_is_a_lasting_failure() {
    assert!(NativeError::Unsupported.is_lasting());
}
