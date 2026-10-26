use super::{Av1SequenceHeaderParts, av1c_of_parts};
use crate::avc::test::video_size;
use crate::video_size_from_av1c;

#[test]
fn a_hand_built_sequence_header_gives_the_size_it_was_built_with() {
    let av1c = av1c_of_parts(&Av1SequenceHeaderParts::default()).expect("the header is read");

    assert_eq!(av1c[..4], [0x81, 0x08, 0x0D, 0x00]);
    assert_eq!(video_size_from_av1c(&av1c), Some(video_size(1920, 1080)));
}
