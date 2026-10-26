use crate::capture::NativeError;

#[test]
fn native_capture_off_windows_says_it_needs_windows() {
    assert_eq!(
        NativeError::Unsupported.to_string(),
        "native capture needs Windows"
    );
}
