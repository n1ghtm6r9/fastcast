use crate::{CursorKind, POINTER_SHAPE_COLOR, decode_cursor_shape};

#[test]
fn a_colour_cursor_keeps_its_alpha() {
    let cursor = decode_cursor_shape(POINTER_SHAPE_COLOR, 1, 1, 4, &[1, 2, 3, 128]).unwrap();

    assert_eq!(cursor.kind, CursorKind::Alpha);
    assert_eq!(cursor.pixels, [1, 2, 3, 128]);
}
