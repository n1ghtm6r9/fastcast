use super::{SpsParts, video_size, video_size_of_parts};

#[test]
fn a_portrait_picture_keeps_width_and_height_apart() {
    let parts = SpsParts {
        width_in_macroblocks: 68,
        height_in_map_units: 120,
        crop: Some([0, 4, 0, 0]),
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1080, 1920)));
}
