use crate::capture::NativeError;
use desktop::DesktopError;
use windows::Win32::Foundation::E_ACCESSDENIED;

#[test]
fn a_desktop_that_cannot_be_duplicated_for_now_is_not_a_lasting_failure() {
    for error in [
        DesktopError::AccessLost,
        DesktopError::AdapterNotFound,
        DesktopError::OddSize {
            width: 1919,
            height: 1080,
        },
        DesktopError::Rotated,
        DesktopError::Api {
            call: "DuplicateOutput",
            source: windows::core::Error::from(E_ACCESSDENIED),
        },
    ] {
        let shown = error.to_string();

        assert!(!NativeError::Desktop(error).is_lasting(), "{shown}");
    }
}
