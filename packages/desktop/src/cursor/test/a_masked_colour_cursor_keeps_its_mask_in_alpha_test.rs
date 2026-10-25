use crate::{CursorKind, POINTER_SHAPE_MASKED_COLOR, decode_cursor_shape};

#[test]
fn a_masked_colour_cursor_keeps_its_mask_in_alpha() {
    let shape = [10, 20, 30, 0, 40, 50, 60, 255];
    let cursor = decode_cursor_shape(POINTER_SHAPE_MASKED_COLOR, 2, 1, 8, &shape).unwrap();

    assert_eq!(cursor.kind, CursorKind::Masked);
    assert_eq!(cursor.pixels, shape);
}
