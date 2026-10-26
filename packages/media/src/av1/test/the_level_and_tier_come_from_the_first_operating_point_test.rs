use super::{Av1SequenceHeaderParts, av1c_of_parts};
use crate::avc::test::video_size;
use crate::{av1_codec_string, video_size_from_av1c};

#[test]
fn the_level_and_tier_come_from_the_first_operating_point() {
    let parts = Av1SequenceHeaderParts {
        operating_points: vec![(13, true), (8, false), (4, false)],
        ..Av1SequenceHeaderParts::default()
    };

    let av1c = av1c_of_parts(&parts).expect("the header is read");

    assert_eq!(av1_codec_string(&av1c), Some("av01.0.13H.08".to_string()));
    assert_eq!(video_size_from_av1c(&av1c), Some(video_size(1920, 1080)));
}
