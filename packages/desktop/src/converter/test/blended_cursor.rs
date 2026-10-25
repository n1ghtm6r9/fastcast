use crate::cursor::{CursorImage, CursorKind};

pub fn blended_cursor() -> CursorImage {
    let alphas = [128, 0, 255, 64, 192];
    let pixels = (0..64usize)
        .flat_map(|index| {
            [
                (index * 37) as u8,
                (index * 91) as u8,
                (index * 53) as u8,
                alphas[index % alphas.len()],
            ]
        })
        .collect();

    CursorImage {
        width: 8,
        height: 8,
        kind: CursorKind::Alpha,
        pixels,
    }
}
