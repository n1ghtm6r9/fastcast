use super::{HevcSpsParts, video_size_of_hevc_parts};
use crate::avc::test::video_size;

#[test]
fn emulation_prevention_bytes_inside_the_sps_are_removed_before_parsing() {
    let parts = HevcSpsParts::default();

    assert!(
        parts
            .nal_unit()
            .windows(3)
            .any(|window| window == [0, 0, 3])
    );
    assert_eq!(
        video_size_of_hevc_parts(&parts),
        Some(video_size(1920, 1080))
    );
}
