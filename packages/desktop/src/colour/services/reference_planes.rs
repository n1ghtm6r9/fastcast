use crate::colour::{BitDepth, Planes, chroma_plane, luma_plane};

pub fn reference_planes(bgra: &[u8], width: usize, height: usize, depth: BitDepth) -> Planes {
    Planes {
        width,
        height,
        luma: luma_plane(bgra, width, height, depth),
        chroma: chroma_plane(bgra, width, height, depth),
    }
}
