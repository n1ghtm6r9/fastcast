use crate::{POINTER_SHAPE_COLOR, POINTER_SHAPE_MONOCHROME, decode_cursor_shape};

#[test]
fn a_shape_pitch_narrower_than_a_row_gives_no_cursor() {
    assert_eq!(
        decode_cursor_shape(POINTER_SHAPE_COLOR, 2, 1, 4, &[0; 4]),
        None
    );
    assert_eq!(
        decode_cursor_shape(POINTER_SHAPE_MONOCHROME, 16, 2, 1, &[0; 2]),
        None
    );
}
