use crate::gpu::{DesktopError, require_even_size};

#[test]
fn an_odd_height_is_rejected() {
    let error = require_even_size(64, 33).unwrap_err();

    assert!(matches!(
        error,
        DesktopError::OddSize {
            width: 64,
            height: 33
        }
    ));
}
