use super::{PictureOrderCountParts, SpsParts, video_size, video_size_of_parts};

#[test]
fn picture_order_count_type_1_skips_its_reference_offsets() {
    let parts = SpsParts {
        picture_order_count: PictureOrderCountParts::Type1 {
            reference_offsets: vec![2, -2, 1, 3],
        },
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1920, 1080)));
}
