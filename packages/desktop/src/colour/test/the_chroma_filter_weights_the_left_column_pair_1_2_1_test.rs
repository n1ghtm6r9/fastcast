use crate::{BitDepth, reference_planes, ycbcr_of};

#[test]
fn the_chroma_filter_weights_the_left_column_pair_1_2_1() {
    let red = [0, 0, 255, 255];
    let blue = [255, 0, 0, 255];
    let row = [red, blue, blue, blue].concat();
    let planes = reference_planes(&[row.clone(), row].concat(), 4, 2, BitDepth::Eight);
    let left = ycbcr_of(0.75, 0.0, 0.25, BitDepth::Eight);
    let right = ycbcr_of(0.0, 0.0, 1.0, BitDepth::Eight);

    assert_eq!(planes.chroma, [left.cb, left.cr, right.cb, right.cr]);
}
