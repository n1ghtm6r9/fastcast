use crate::converter::cursor_is_complete;
use crate::cursor::{CursorImage, CursorKind};

#[test]
fn a_cursor_with_missing_pixels_is_rejected() {
    let cursor = CursorImage {
        width: 2,
        height: 3,
        kind: CursorKind::Alpha,
        pixels: vec![0; 23],
    };

    assert!(!cursor_is_complete(&cursor));
}
