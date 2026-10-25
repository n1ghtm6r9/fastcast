use crate::layout::MAX_FRAME_SIZE;
use crate::packetizer::test::video_frame;
use crate::packetizer::{Datagrams, Packetizer};

#[test]
fn a_frame_over_the_size_limit_is_skipped_but_uses_its_number() {
    let oversized = vec![0u8; MAX_FRAME_SIZE + 1];
    let small = vec![0u8; 100];
    let mut packetizer = Packetizer::new(1);
    let mut out = Datagrams::new();

    let skipped = packetizer.frame(&video_frame(&oversized, true), 10, &mut out);
    let skipped_was_empty = out.is_empty();
    let next = packetizer.frame(&video_frame(&small, true), 10, &mut out);

    assert_eq!((skipped, skipped_was_empty, next), (0, true, 1));
}
