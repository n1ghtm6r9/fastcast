use crate::{CursorImage, CursorKind, composite_cursor};

#[test]
fn an_inverting_cursor_flips_the_screen_under_it() {
    let mut screen = vec![10, 20, 30, 255, 200, 100, 50, 255];
    let cursor = CursorImage {
        width: 1,
        height: 1,
        kind: CursorKind::Masked,
        pixels: vec![255, 255, 255, 255],
    };

    composite_cursor(&mut screen, 2, 1, &cursor, 1, 0);

    assert_eq!(screen, [10, 20, 30, 255, 55, 155, 205, 255]);
}
