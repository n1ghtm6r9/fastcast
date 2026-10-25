use crate::cursor::{CursorImage, CursorKind};

pub fn masked_cursor() -> CursorImage {
    let patterns: [[u8; 4]; 5] = [
        [0xFF, 0xFF, 0xFF, 0xFF],
        [0x00, 0x00, 0x00, 0xFF],
        [0x00, 0x00, 0x00, 0x00],
        [0x40, 0x80, 0xC0, 0xFF],
        [0x12, 0x34, 0x56, 0x7F],
    ];
    let pixels = (0..64)
        .flat_map(|index| patterns[index % patterns.len()])
        .collect();

    CursorImage {
        width: 8,
        height: 8,
        kind: CursorKind::Masked,
        pixels,
    }
}
