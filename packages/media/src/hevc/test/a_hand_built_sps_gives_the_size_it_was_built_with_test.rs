use super::{HevcSpsParts, video_size_of_hevc_parts};
use crate::avc::test::video_size;

#[test]
fn a_hand_built_sps_gives_the_size_it_was_built_with() {
    assert_eq!(
        video_size_of_hevc_parts(&HevcSpsParts::default()),
        Some(video_size(1920, 1080))
    );
}
