use super::{HevcSpsParts, hvcc_with_arrays};
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn the_first_sps_is_used_when_the_hvcc_carries_several() {
    let first = HevcSpsParts::default().nal_unit();
    let second = HevcSpsParts {
        width_in_luma_samples: 1280,
        height_in_luma_samples: 720,
        conformance_window: None,
        ..HevcSpsParts::default()
    }
    .nal_unit();
    let hvcc = hvcc_with_arrays(&[(33, vec![first, second])]);

    assert_eq!(video_size_from_hvcc(&hvcc), Some(video_size(1920, 1080)));
}
