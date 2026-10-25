use crate::{POINTER_SHAPE_COLOR, decode_cursor_shape};

#[test]
fn a_zero_sized_shape_gives_no_cursor() {
    assert_eq!(
        decode_cursor_shape(POINTER_SHAPE_COLOR, 0, 1, 4, &[0; 4]),
        None
    );
    assert_eq!(
        decode_cursor_shape(POINTER_SHAPE_COLOR, 1, 0, 4, &[0; 4]),
        None
    );
}
