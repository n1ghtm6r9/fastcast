use super::{SpsParts, avcc_with_sps_units, video_size};
use crate::video_size_from_avcc;

#[test]
fn the_first_sps_is_used_when_the_avcc_carries_several() {
    let first = SpsParts::default().nal_unit();
    let second = SpsParts {
        width_in_macroblocks: 80,
        height_in_map_units: 45,
        crop: None,
        ..SpsParts::default()
    }
    .nal_unit();
    let avcc = avcc_with_sps_units(&[first, second]);

    assert_eq!(video_size_from_avcc(&avcc), Some(video_size(1920, 1080)));
}
