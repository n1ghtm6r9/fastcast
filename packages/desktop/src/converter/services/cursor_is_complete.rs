use crate::cursor::{BYTES_PER_PIXEL, CursorImage};

pub fn cursor_is_complete(cursor: &CursorImage) -> bool {
    let expected = (cursor.width as usize)
        .checked_mul(cursor.height as usize)
        .and_then(|pixels| pixels.checked_mul(BYTES_PER_PIXEL));

    cursor.width > 0 && cursor.height > 0 && expected == Some(cursor.pixels.len())
}
