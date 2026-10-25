use super::black_screen;
use crate::{CursorImage, CursorKind, composite_cursor};

#[test]
fn a_cursor_beyond_every_edge_changes_nothing() {
    let mut screen = black_screen(2, 2);
    let cursor = CursorImage {
        width: 1,
        height: 1,
        kind: CursorKind::Alpha,
        pixels: vec![9, 9, 9, 255],
    };

    for (left, top) in [
        (i32::MIN, 0),
        (i32::MAX, 0),
        (0, i32::MIN),
        (0, i32::MAX),
        (-1, -1),
        (2, 2),
    ] {
        composite_cursor(&mut screen, 2, 2, &cursor, left, top);
    }

    assert_eq!(screen, black_screen(2, 2));
}
