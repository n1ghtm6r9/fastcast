use super::white_over_black;
use crate::{BitDepth, reference_planes};

#[test]
fn luma_is_computed_per_pixel() {
    let planes = reference_planes(&white_over_black(), 2, 2, BitDepth::Eight);

    assert_eq!(planes.luma, [235, 235, 16, 16]);
}
