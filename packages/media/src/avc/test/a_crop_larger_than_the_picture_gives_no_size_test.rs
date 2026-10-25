use super::{SpsParts, video_size_of_parts};

#[test]
fn a_crop_larger_than_the_picture_gives_no_size() {
    let parts = SpsParts {
        height_in_map_units: 1,
        crop: Some([0, 0, 0, 9]),
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), None);
}
