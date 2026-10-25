use super::{PictureOrderCountParts, SpsParts, video_size, video_size_of_parts};

#[test]
fn emulation_prevention_bytes_inside_the_sps_are_removed_before_parsing() {
    let parts = SpsParts {
        picture_order_count: PictureOrderCountParts::Type1 {
            reference_offsets: vec![8_388_608],
        },
        ..SpsParts::default()
    };

    assert!(
        parts
            .nal_unit()
            .windows(3)
            .any(|window| window == [0, 0, 3])
    );
    assert_eq!(video_size_of_parts(&parts), Some(video_size(1920, 1080)));
}
