use crate::colour::{BitDepth, Planes, reference_planes};
use crate::cursor::{CursorImage, composite_cursor};

pub fn reference_with_cursor(
    bgra: &[u8],
    width: usize,
    height: usize,
    depth: BitDepth,
    cursor: &CursorImage,
    left: i32,
    top: i32,
) -> Planes {
    let mut composited = bgra.to_vec();

    composite_cursor(&mut composited, width, height, cursor, left, top);
    reference_planes(&composited, width, height, depth)
}
