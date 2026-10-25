use crate::packetizer::test::{shard_of, video_frame};
use crate::packetizer::{Datagrams, Packetizer};

#[test]
fn frame_numbers_count_frames() {
    let payload = vec![0u8; 100];
    let mut packetizer = Packetizer::new(7);
    let mut out = Datagrams::new();

    let first = packetizer.frame(&video_frame(&payload, true), 10, &mut out);
    let first_header_frame = shard_of(out.get(0)).header.frame;
    let second = packetizer.frame(&video_frame(&payload, false), 10, &mut out);
    let second_header_frame = shard_of(out.get(0)).header.frame;

    assert_eq!(
        (first, first_header_frame, second, second_header_frame),
        (0, 0, 1, 1)
    );
}
