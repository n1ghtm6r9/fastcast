use crate::capture::NativeError;
use desktop::DesktopError;
use std::error::Error;

#[test]
fn a_reopen_that_gave_up_keeps_its_cause_as_the_source() {
    let error = NativeError::ReopenGaveUp(DesktopError::Rotated);

    assert_eq!(
        error
            .source()
            .expect("the desktop error is the source")
            .to_string(),
        DesktopError::Rotated.to_string()
    );
}
