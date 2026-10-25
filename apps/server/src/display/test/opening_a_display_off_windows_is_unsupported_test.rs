use super::mode_of;
use crate::display::{DisplayError, VirtualDisplay};

#[test]
fn opening_a_display_off_windows_is_unsupported() {
    let error = VirtualDisplay::open(mode_of(3840, 2160, 60))
        .err()
        .expect("opening is refused");

    assert!(matches!(error, DisplayError::Unsupported));
}
