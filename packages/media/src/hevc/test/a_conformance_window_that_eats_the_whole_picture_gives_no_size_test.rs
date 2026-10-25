use super::{HevcSpsParts, video_size_of_hevc_parts};

#[test]
fn a_conformance_window_that_eats_the_whole_picture_gives_no_size() {
    let parts = HevcSpsParts {
        width_in_luma_samples: 16,
        conformance_window: Some([4, 4, 0, 0]),
        ..HevcSpsParts::default()
    };

    assert_eq!(video_size_of_hevc_parts(&parts), None);
}
