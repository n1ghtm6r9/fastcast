use super::{HevcSpsParts, SubLayerParts, video_size_of_hevc_parts};
use crate::avc::test::video_size;

#[test]
fn six_sub_layers_with_every_field_are_skipped_whole() {
    let full = SubLayerParts {
        profile_present: true,
        level_present: true,
    };
    let parts = HevcSpsParts {
        sub_layers: vec![full; 6],
        ..HevcSpsParts::default()
    };

    assert_eq!(
        video_size_of_hevc_parts(&parts),
        Some(video_size(1920, 1080))
    );
}
