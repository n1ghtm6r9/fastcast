use super::{HevcSpsParts, video_size_of_hevc_parts};
use crate::avc::test::video_size;

#[test]
fn a_portrait_picture_keeps_width_and_height_apart() {
    let parts = HevcSpsParts {
        width_in_luma_samples: 1088,
        height_in_luma_samples: 1920,
        conformance_window: Some([0, 4, 0, 0]),
        ..HevcSpsParts::default()
    };

    assert_eq!(
        video_size_of_hevc_parts(&parts),
        Some(video_size(1080, 1920))
    );
}
