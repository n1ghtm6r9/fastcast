use crate::{POINTER_SHAPE_COLOR, decode_cursor_shape};

#[test]
fn rows_are_read_with_the_shape_pitch() {
    let shape = [1, 2, 3, 4, 9, 9, 9, 9, 5, 6, 7, 8, 9, 9, 9, 9];
    let cursor = decode_cursor_shape(POINTER_SHAPE_COLOR, 1, 2, 8, &shape).unwrap();

    assert_eq!(cursor.pixels, [1, 2, 3, 4, 5, 6, 7, 8]);
}
