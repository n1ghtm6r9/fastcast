use super::{HevcSpsParts, video_size_of_hevc_parts};

#[test]
fn a_picture_without_any_samples_gives_no_size() {
    let parts = HevcSpsParts {
        width_in_luma_samples: 0,
        conformance_window: None,
        ..HevcSpsParts::default()
    };

    assert_eq!(video_size_of_hevc_parts(&parts), None);
}
