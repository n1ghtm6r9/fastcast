use super::white_over_black;
use crate::{BitDepth, reference_planes};

#[test]
fn rows_are_averaged_for_chroma() {
    let planes = reference_planes(&white_over_black(), 2, 2, BitDepth::Eight);

    assert_eq!(planes.chroma, [128, 128]);
}
