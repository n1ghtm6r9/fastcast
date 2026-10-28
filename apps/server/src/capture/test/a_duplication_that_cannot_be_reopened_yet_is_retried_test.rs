use crate::capture::{ReopenStep, reopen_step_of};
use desktop::DesktopError;
use windows::Win32::Foundation::E_ACCESSDENIED;

#[test]
fn a_duplication_that_cannot_be_reopened_yet_is_retried() {
    let error = DesktopError::Api {
        call: "IDXGIOutput1::DuplicateOutput",
        source: windows::core::Error::from(E_ACCESSDENIED),
    };

    assert_eq!(reopen_step_of((3840, 2160), Err(&error)), ReopenStep::Retry);
}
