use super::{HevcSpsParts, video_size_of_hevc_parts};
use crate::avc::test::video_size;

#[test]
fn the_conformance_window_is_applied_on_all_four_sides() {
    let parts = HevcSpsParts {
        conformance_window: Some([1, 2, 3, 4]),
        ..HevcSpsParts::default()
    };

    assert_eq!(
        video_size_of_hevc_parts(&parts),
        Some(video_size(1914, 1074))
    );
}
