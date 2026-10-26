use crate::capture::NativeError;
use desktop::DesktopError;

#[test]
fn a_reopen_that_gave_up_is_not_a_lasting_failure() {
    assert!(!NativeError::ReopenGaveUp(DesktopError::AccessLost).is_lasting());
}
