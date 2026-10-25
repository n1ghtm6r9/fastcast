use super::{PictureOrderCountParts, SpsParts, video_size, video_size_of_parts};

#[test]
fn picture_order_count_type_2_has_no_extra_fields() {
    let parts = SpsParts {
        picture_order_count: PictureOrderCountParts::Type2,
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1920, 1080)));
}
