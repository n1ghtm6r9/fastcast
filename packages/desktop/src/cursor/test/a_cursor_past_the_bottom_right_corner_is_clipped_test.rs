use super::black_screen;
use crate::{CursorImage, CursorKind, composite_cursor};

#[test]
fn a_cursor_past_the_bottom_right_corner_is_clipped() {
    let mut screen = black_screen(2, 2);
    let cursor = CursorImage {
        width: 2,
        height: 2,
        kind: CursorKind::Alpha,
        pixels: [
            [1, 1, 1, 255],
            [2, 2, 2, 255],
            [3, 3, 3, 255],
            [4, 4, 4, 255],
        ]
        .concat(),
    };

    composite_cursor(&mut screen, 2, 2, &cursor, 1, 1);

    assert_eq!(
        screen,
        [
            [0, 0, 0, 255],
            [0, 0, 0, 255],
            [0, 0, 0, 255],
            [1, 1, 1, 255]
        ]
        .concat()
    );
}
