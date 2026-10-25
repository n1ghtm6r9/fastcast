use crate::{CursorImage, CursorKind, composite_cursor};

#[test]
fn a_half_transparent_cursor_blends_with_the_screen() {
    let mut screen = vec![0, 0, 0, 255];
    let cursor = CursorImage {
        width: 1,
        height: 1,
        kind: CursorKind::Alpha,
        pixels: vec![255, 255, 255, 128],
    };

    composite_cursor(&mut screen, 1, 1, &cursor, 0, 0);

    assert_eq!(screen, [128, 128, 128, 255]);
}
