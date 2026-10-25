use super::{HevcSpsParts, SubLayerParts, video_size_of_hevc_parts};
use crate::avc::test::video_size;

#[test]
fn sub_layers_carrying_profile_and_level_fields_are_skipped_whole() {
    let parts = HevcSpsParts {
        sub_layers: vec![
            SubLayerParts {
                profile_present: true,
                level_present: true,
            },
            SubLayerParts {
                profile_present: true,
                level_present: false,
            },
            SubLayerParts {
                profile_present: false,
                level_present: true,
            },
        ],
        ..HevcSpsParts::default()
    };

    assert_eq!(
        video_size_of_hevc_parts(&parts),
        Some(video_size(1920, 1080))
    );
}
