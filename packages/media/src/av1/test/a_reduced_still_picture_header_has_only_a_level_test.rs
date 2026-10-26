use super::{Av1SequenceHeaderParts, av1c_of_parts};
use crate::avc::test::video_size;
use crate::{av1_codec_string, video_size_from_av1c};

#[test]
fn a_reduced_still_picture_header_has_only_a_level() {
    let parts = Av1SequenceHeaderParts {
        reduced_still_picture_header: true,
        operating_points: vec![(4, false)],
        ..Av1SequenceHeaderParts::default()
    };

    let av1c = av1c_of_parts(&parts).expect("the header is read");

    assert_eq!(av1c[..4], [0x81, 0x04, 0x0D, 0x00]);
    assert_eq!(av1_codec_string(&av1c), Some("av01.0.04M.08".to_string()));
    assert_eq!(video_size_from_av1c(&av1c), Some(video_size(1920, 1080)));
}
