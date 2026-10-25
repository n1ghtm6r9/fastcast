use crate::duplication::duplication_error;
use crate::gpu::DesktopError;
use windows::Win32::Foundation::E_FAIL;

#[test]
fn other_failures_keep_the_failed_call_and_the_cause() {
    let error = duplication_error("IDXGIOutputDuplication::AcquireNextFrame", E_FAIL.into());

    match error {
        DesktopError::Api { call, source } => {
            assert_eq!(call, "IDXGIOutputDuplication::AcquireNextFrame");
            assert_eq!(source.code(), E_FAIL);
        }
        other => panic!("unexpected error: {other}"),
    }
}
