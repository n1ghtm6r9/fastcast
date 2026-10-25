use super::{PictureOrderCountParts, SpsParts, video_size_of_parts};

#[test]
fn an_unknown_picture_order_count_type_gives_no_size() {
    let parts = SpsParts {
        picture_order_count: PictureOrderCountParts::Unknown(3),
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), None);
}
