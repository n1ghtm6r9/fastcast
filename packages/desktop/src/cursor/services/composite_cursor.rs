use crate::cursor::{
    BYTES_PER_PIXEL, CursorImage, CursorKind, apply_masked_pixel, blend_alpha_pixel, visible_range,
};

pub fn composite_cursor(
    bgra: &mut [u8],
    width: usize,
    height: usize,
    cursor: &CursorImage,
    left: i32,
    top: i32,
) {
    let columns = visible_range(left, cursor.width, width);
    let rows = visible_range(top, cursor.height, height);

    for row in rows {
        for column in columns.clone() {
            let screen_column = (i64::from(left) + column as i64) as usize;
            let screen_row = (i64::from(top) + row as i64) as usize;
            let screen_offset = (screen_row * width + screen_column) * BYTES_PER_PIXEL;
            let cursor_offset = (row * cursor.width as usize + column) * BYTES_PER_PIXEL;
            let screen_pixel = &mut bgra[screen_offset..screen_offset + BYTES_PER_PIXEL];
            let cursor_pixel = &cursor.pixels[cursor_offset..cursor_offset + BYTES_PER_PIXEL];

            match cursor.kind {
                CursorKind::Masked => apply_masked_pixel(screen_pixel, cursor_pixel),
                CursorKind::Alpha => blend_alpha_pixel(screen_pixel, cursor_pixel),
            }
        }
    }
}
