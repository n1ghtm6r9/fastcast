use super::{PictureOrderCountParts, SpsParts, video_size, video_size_of_parts};

#[test]
fn picture_order_count_type_1_without_reference_offsets_is_skipped() {
    let parts = SpsParts {
        picture_order_count: PictureOrderCountParts::Type1 {
            reference_offsets: Vec::new(),
        },
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1920, 1080)));
}
