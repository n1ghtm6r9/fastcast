use crate::{POINTER_SHAPE_COLOR, decode_cursor_shape};

#[test]
fn a_short_buffer_gives_no_cursor() {
    assert_eq!(
        decode_cursor_shape(POINTER_SHAPE_COLOR, 2, 2, 8, &[0; 15]),
        None
    );
}
