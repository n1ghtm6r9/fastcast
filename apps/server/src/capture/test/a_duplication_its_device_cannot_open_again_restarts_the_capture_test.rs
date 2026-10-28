use crate::capture::{ReopenStep, reopen_step_of};
use desktop::DesktopError;
use windows::Win32::Foundation::E_FAIL;

#[test]
fn a_duplication_its_device_cannot_open_again_restarts_the_capture() {
    let error = DesktopError::Api {
        call: "IDXGIOutput1::DuplicateOutput",
        source: windows::core::Error::from(E_FAIL),
    };

    assert_eq!(
        reopen_step_of((3840, 2160), Err(&error)),
        ReopenStep::Restart
    );
}
