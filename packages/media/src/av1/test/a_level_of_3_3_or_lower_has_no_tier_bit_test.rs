use super::{Av1SequenceHeaderParts, av1c_of_parts};
use crate::avc::test::video_size;
use crate::{av1_codec_string, video_size_from_av1c};

#[test]
fn a_level_of_3_3_or_lower_has_no_tier_bit() {
    let parts = Av1SequenceHeaderParts {
        operating_points: vec![(7, false), (12, true)],
        ..Av1SequenceHeaderParts::default()
    };

    let av1c = av1c_of_parts(&parts).expect("the header is read");

    assert_eq!(av1_codec_string(&av1c), Some("av01.0.07M.08".to_string()));
    assert_eq!(video_size_from_av1c(&av1c), Some(video_size(1920, 1080)));
}
