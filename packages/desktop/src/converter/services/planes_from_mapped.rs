use crate::colour::{BitDepth, Planes};
use crate::converter::plane_samples;

pub fn planes_from_mapped(
    mapped: &[u8],
    row_pitch: usize,
    width: usize,
    height: usize,
    depth: BitDepth,
) -> Planes {
    Planes {
        width,
        height,
        luma: plane_samples(mapped, 0, row_pitch, width, height, depth),
        chroma: plane_samples(
            mapped,
            row_pitch * height,
            row_pitch,
            width,
            height / 2,
            depth,
        ),
    }
}
