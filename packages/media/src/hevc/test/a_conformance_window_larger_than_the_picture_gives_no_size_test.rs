use super::{HevcSpsParts, video_size_of_hevc_parts};

#[test]
fn a_conformance_window_larger_than_the_picture_gives_no_size() {
    let parts = HevcSpsParts {
        height_in_luma_samples: 16,
        conformance_window: Some([0, 0, 0, 9]),
        ..HevcSpsParts::default()
    };

    assert_eq!(video_size_of_hevc_parts(&parts), None);
}
