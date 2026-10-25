use crate::converter::cursor_is_complete;
use crate::cursor::{CursorImage, CursorKind};

#[test]
fn a_whole_cursor_is_accepted() {
    let cursor = CursorImage {
        width: 2,
        height: 3,
        kind: CursorKind::Alpha,
        pixels: vec![0; 24],
    };

    assert!(cursor_is_complete(&cursor));
}
