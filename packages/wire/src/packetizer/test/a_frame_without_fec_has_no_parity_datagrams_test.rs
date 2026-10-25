use crate::packetizer::test::{shard_of, video_frame};
use crate::packetizer::{Datagrams, Packetizer};

#[test]
fn a_frame_without_fec_has_no_parity_datagrams() {
    let payload = vec![1u8; 3000];
    let mut packetizer = Packetizer::new(1);
    let mut out = Datagrams::new();

    packetizer.frame(&video_frame(&payload, false), 0, &mut out);

    assert_eq!(out.len(), 3);
    assert!(
        out.iter()
            .all(|datagram| shard_of(datagram).header.parity_shards == 0)
    );
}
