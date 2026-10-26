use crate::capture::NativeError;
use desktop::DesktopError;

#[test]
fn a_reopen_that_gave_up_says_how_long_and_why() {
    let error = NativeError::ReopenGaveUp(DesktopError::AdapterNotFound);

    assert_eq!(
        error.to_string(),
        format!(
            "the desktop could not be duplicated again for 5 seconds: {}",
            DesktopError::AdapterNotFound
        )
    );
}
