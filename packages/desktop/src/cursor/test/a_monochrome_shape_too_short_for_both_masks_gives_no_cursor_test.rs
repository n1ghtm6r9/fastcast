use crate::{POINTER_SHAPE_MONOCHROME, decode_cursor_shape};

#[test]
fn a_monochrome_shape_too_short_for_both_masks_gives_no_cursor() {
    assert_eq!(
        decode_cursor_shape(POINTER_SHAPE_MONOCHROME, 8, 1, 1, &[0xFF]),
        None
    );
}
