use crate::capture::NativeError;
use desktop::DesktopError;
use std::error::Error;

#[test]
fn a_desktop_error_is_shown_as_it_is_and_kept_as_the_source() {
    let error = NativeError::Desktop(DesktopError::AccessLost);

    assert_eq!(error.to_string(), DesktopError::AccessLost.to_string());
    assert_eq!(
        error
            .source()
            .expect("the desktop error is the source")
            .to_string(),
        DesktopError::AccessLost.to_string()
    );
}
