use super::{Av1SequenceHeaderParts, av1c_of_parts};
use crate::avc::test::video_size;
use crate::video_size_from_av1c;

#[test]
fn timing_info_without_an_equal_picture_interval_is_read_past() {
    let parts = Av1SequenceHeaderParts {
        timing_info: Some(false),
        high_bitdepth: true,
        ..Av1SequenceHeaderParts::default()
    };

    let av1c = av1c_of_parts(&parts).expect("the header is read");

    assert_eq!(av1c[..4], [0x81, 0x08, 0x4D, 0x00]);
    assert_eq!(video_size_from_av1c(&av1c), Some(video_size(1920, 1080)));
}
