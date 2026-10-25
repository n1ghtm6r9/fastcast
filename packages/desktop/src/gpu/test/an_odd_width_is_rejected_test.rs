use crate::gpu::{DesktopError, require_even_size};

#[test]
fn an_odd_width_is_rejected() {
    let error = require_even_size(63, 32).unwrap_err();

    assert!(matches!(
        error,
        DesktopError::OddSize {
            width: 63,
            height: 32
        }
    ));
}
