use crate::cursor::{BYTES_PER_PIXEL, CursorImage, CursorKind};

pub fn decode_colour_shape(
    kind: CursorKind,
    width: u32,
    height: u32,
    pitch: usize,
    buffer: &[u8],
) -> Option<CursorImage> {
    let row_size = (width as usize).checked_mul(BYTES_PER_PIXEL)?;

    if pitch < row_size {
        return None;
    }

    let mut pixels = Vec::with_capacity(row_size * height as usize);

    for row in buffer.chunks(pitch).take(height as usize) {
        pixels.extend_from_slice(&row[..row_size]);
    }

    Some(CursorImage {
        width,
        height,
        kind,
        pixels,
    })
}
