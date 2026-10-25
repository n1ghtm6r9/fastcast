use crate::colour::{BitDepth, filtered_row_rgb, ycbcr_of};

pub fn chroma_plane(bgra: &[u8], width: usize, height: usize, depth: BitDepth) -> Vec<u16> {
    let mut chroma = Vec::with_capacity(width / 2 * (height / 2) * 2);

    for chroma_row in 0..height / 2 {
        for chroma_column in 0..width / 2 {
            let upper = filtered_row_rgb(bgra, width, chroma_row * 2, chroma_column);
            let lower = filtered_row_rgb(bgra, width, chroma_row * 2 + 1, chroma_column);
            let [r, g, b] = std::array::from_fn(|channel| (upper[channel] + lower[channel]) / 2.0);
            let sample = ycbcr_of(r, g, b, depth);

            chroma.push(sample.cb);
            chroma.push(sample.cr);
        }
    }

    chroma
}
