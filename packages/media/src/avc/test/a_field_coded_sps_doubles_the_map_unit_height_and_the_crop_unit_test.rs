use super::{SpsParts, video_size, video_size_of_parts};

#[test]
fn a_field_coded_sps_doubles_the_map_unit_height_and_the_crop_unit() {
    let parts = SpsParts {
        height_in_map_units: 34,
        frame_mbs_only: false,
        crop: Some([0, 0, 0, 2]),
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1920, 1080)));
}
