use crate::cursor::{BYTES_PER_PIXEL, CursorImage, CursorKind, MASK_KEEPS_SCREEN, monochrome_bit};

pub fn decode_monochrome_shape(
    width: u32,
    height: u32,
    pitch: usize,
    buffer: &[u8],
) -> Option<CursorImage> {
    let cursor_height = (height / 2) as usize;
    let columns = width as usize;
    let row_size = columns.div_ceil(8);

    if cursor_height == 0 || pitch < row_size {
        return None;
    }

    let mut pixels = Vec::with_capacity(columns * cursor_height * BYTES_PER_PIXEL);

    for row in 0..cursor_height {
        let and_row = &buffer[row * pitch..][..row_size];
        let xor_row = &buffer[(row + cursor_height) * pitch..][..row_size];

        for column in 0..columns {
            let alpha = if monochrome_bit(and_row, column) {
                MASK_KEEPS_SCREEN
            } else {
                0
            };
            let colour = if monochrome_bit(xor_row, column) {
                u8::MAX
            } else {
                0
            };

            pixels.extend_from_slice(&[colour, colour, colour, alpha]);
        }
    }

    Some(CursorImage {
        width,
        height: height / 2,
        kind: CursorKind::Masked,
        pixels,
    })
}
