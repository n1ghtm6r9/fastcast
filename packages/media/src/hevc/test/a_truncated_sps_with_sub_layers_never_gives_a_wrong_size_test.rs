use super::{HevcSpsParts, SubLayerParts};
use crate::avc::test::video_size;
use crate::hevc::video_size_from_hevc_sps;

#[test]
fn a_truncated_sps_with_sub_layers_never_gives_a_wrong_size() {
    let full = SubLayerParts {
        profile_present: true,
        level_present: true,
    };
    let parts = HevcSpsParts {
        sub_layers: vec![full; 3],
        ..HevcSpsParts::default()
    };
    let nal_unit = parts.nal_unit();
    let expected = video_size(1920, 1080);

    for length in 0..=nal_unit.len() {
        let size = video_size_from_hevc_sps(&nal_unit[..length]);

        assert!(
            size.is_none() || size == Some(expected),
            "{length} bytes gave {size:?}"
        );

        if length <= 50 {
            assert_eq!(size, None, "{length} bytes");
        }
    }
}
