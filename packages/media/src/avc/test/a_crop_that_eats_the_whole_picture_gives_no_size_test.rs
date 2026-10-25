use super::{SpsParts, video_size_of_parts};

#[test]
fn a_crop_that_eats_the_whole_picture_gives_no_size() {
    let parts = SpsParts {
        width_in_macroblocks: 1,
        crop: Some([0, 8, 0, 0]),
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), None);
}
