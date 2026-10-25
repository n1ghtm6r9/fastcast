use crate::{CursorKind, POINTER_SHAPE_MONOCHROME, decode_cursor_shape};

#[test]
fn a_monochrome_cursor_reads_and_then_xor_rows() {
    let cursor = decode_cursor_shape(
        POINTER_SHAPE_MONOCHROME,
        8,
        2,
        1,
        &[0b0011_1111, 0b0101_0000],
    )
    .unwrap();

    assert_eq!(
        (cursor.width, cursor.height, cursor.kind),
        (8, 1, CursorKind::Masked)
    );

    let kept = [0, 0, 0, 255];

    assert_eq!(
        cursor.pixels,
        [
            [0, 0, 0, 0],
            [255, 255, 255, 0],
            kept,
            [255, 255, 255, 255],
            kept,
            kept,
            kept,
            kept
        ]
        .concat()
    );
}
