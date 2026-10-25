use crate::packet::SHARD_SIZE;
use crate::packetizer::test::{shard_of, video_frame};
use crate::packetizer::{Datagrams, Packetizer};

#[test]
fn datagrams_alternate_between_blocks() {
    let payload = vec![1u8; 129 * SHARD_SIZE];
    let mut packetizer = Packetizer::new(1);
    let mut out = Datagrams::new();

    packetizer.frame(&video_frame(&payload, true), 0, &mut out);

    let order: Vec<(u16, u8)> = out
        .iter()
        .take(4)
        .map(|datagram| {
            let header = shard_of(datagram).header;
            (header.block, header.shard)
        })
        .collect();
    assert_eq!(out.len(), 129);
    assert_eq!(order, [(0, 0), (1, 0), (0, 1), (1, 1)]);
}
