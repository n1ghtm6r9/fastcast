use super::black_screen;
use crate::{CursorImage, CursorKind, composite_cursor};

#[test]
fn a_cursor_off_the_left_edge_is_clipped() {
    let mut screen = black_screen(2, 1);
    let cursor = CursorImage {
        width: 2,
        height: 1,
        kind: CursorKind::Alpha,
        pixels: vec![1, 1, 1, 255, 2, 2, 2, 255],
    };

    composite_cursor(&mut screen, 2, 1, &cursor, -1, 0);

    assert_eq!(screen, [2, 2, 2, 255, 0, 0, 0, 255]);
}
