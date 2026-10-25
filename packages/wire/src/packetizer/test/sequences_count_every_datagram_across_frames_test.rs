use crate::packetizer::test::{sequence_of, video_frame};
use crate::packetizer::{Datagrams, Packetizer};

#[test]
fn sequences_count_every_datagram_across_frames() {
    let payload = vec![0u8; 100];
    let mut packetizer = Packetizer::new(7);
    let mut out = Datagrams::new();
    let mut sequences = Vec::new();

    for _ in 0..2 {
        packetizer.frame(&video_frame(&payload, false), 10, &mut out);
        sequences.extend(out.iter().map(sequence_of));
    }

    assert_eq!(sequences, [0, 1, 2, 3]);
}
