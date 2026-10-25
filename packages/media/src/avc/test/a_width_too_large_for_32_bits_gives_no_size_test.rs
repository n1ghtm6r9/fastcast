use super::{SpsParts, video_size_of_parts};

#[test]
fn a_width_too_large_for_32_bits_gives_no_size() {
    let parts = SpsParts {
        width_in_macroblocks: u32::MAX,
        crop: None,
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), None);
}
