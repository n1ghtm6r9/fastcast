use crate::colour::{BitDepth, unit_rgb, ycbcr_of};

pub fn luma_plane(bgra: &[u8], width: usize, height: usize, depth: BitDepth) -> Vec<u16> {
    let mut luma = Vec::with_capacity(width * height);

    for row in 0..height {
        for column in 0..width {
            let [r, g, b] = unit_rgb(bgra, width, column, row);

            luma.push(ycbcr_of(r, g, b, depth).y);
        }
    }

    luma
}
