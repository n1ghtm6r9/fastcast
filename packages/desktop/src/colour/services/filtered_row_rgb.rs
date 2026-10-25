use crate::colour::{CHROMA_CENTRE_TAP_WEIGHT, CHROMA_EDGE_TAP_WEIGHT, unit_rgb};

pub fn filtered_row_rgb(bgra: &[u8], width: usize, row: usize, chroma_column: usize) -> [f32; 3] {
    let centre = chroma_column * 2;
    let left = unit_rgb(bgra, width, centre.saturating_sub(1), row);
    let middle = unit_rgb(bgra, width, centre, row);
    let right = unit_rgb(bgra, width, (centre + 1).min(width - 1), row);

    std::array::from_fn(|channel| {
        CHROMA_EDGE_TAP_WEIGHT * left[channel]
            + CHROMA_CENTRE_TAP_WEIGHT * middle[channel]
            + CHROMA_EDGE_TAP_WEIGHT * right[channel]
    })
}
