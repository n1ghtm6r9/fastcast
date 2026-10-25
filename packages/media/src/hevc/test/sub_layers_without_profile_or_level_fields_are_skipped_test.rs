use super::{HevcSpsParts, SubLayerParts, video_size_of_hevc_parts};
use crate::avc::test::video_size;

#[test]
fn sub_layers_without_profile_or_level_fields_are_skipped() {
    let absent = SubLayerParts {
        profile_present: false,
        level_present: false,
    };
    let parts = HevcSpsParts {
        sub_layers: vec![absent; 2],
        ..HevcSpsParts::default()
    };

    assert_eq!(
        video_size_of_hevc_parts(&parts),
        Some(video_size(1920, 1080))
    );
}
