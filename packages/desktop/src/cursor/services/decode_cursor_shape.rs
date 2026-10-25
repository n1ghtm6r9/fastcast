use crate::cursor::{
    CursorImage, CursorKind, POINTER_SHAPE_COLOR, POINTER_SHAPE_MASKED_COLOR,
    POINTER_SHAPE_MONOCHROME, decode_colour_shape, decode_monochrome_shape,
};

pub fn decode_cursor_shape(
    shape_type: u32,
    width: u32,
    height: u32,
    pitch: u32,
    buffer: &[u8],
) -> Option<CursorImage> {
    if width == 0 || height == 0 {
        return None;
    }

    let pitch = pitch as usize;

    if buffer.len() < pitch.checked_mul(height as usize)? {
        return None;
    }

    match shape_type {
        POINTER_SHAPE_MONOCHROME => decode_monochrome_shape(width, height, pitch, buffer),
        POINTER_SHAPE_COLOR => decode_colour_shape(CursorKind::Alpha, width, height, pitch, buffer),
        POINTER_SHAPE_MASKED_COLOR => {
            decode_colour_shape(CursorKind::Masked, width, height, pitch, buffer)
        }
        _ => None,
    }
}
