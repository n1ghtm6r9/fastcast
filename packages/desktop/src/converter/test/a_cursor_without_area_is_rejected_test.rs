use crate::converter::cursor_is_complete;
use crate::cursor::{CursorImage, CursorKind};

#[test]
fn a_cursor_without_area_is_rejected() {
    let cursor = CursorImage {
        width: 0,
        height: 3,
        kind: CursorKind::Masked,
        pixels: Vec::new(),
    };

    assert!(!cursor_is_complete(&cursor));
}
