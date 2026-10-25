use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::packetizer::test::shard_of;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use crate::test_support::random_bytes;
use std::time::Instant;

#[test]
fn a_shard_with_another_parity_count_than_its_block_is_ignored() {
    let mut state = 31;
    let payload = random_bytes(&mut state, 3000);
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();
    let datagrams = datagrams_of(&mut packetizer, FrameKind::Video, true, &payload, 50);
    let mut disagreeing = shard_of(&datagrams[3]);
    disagreeing.header.parity_shards = 1;

    feed(&mut receiver, &datagrams[0], t0);
    feed(&mut receiver, &datagrams[1], t0);
    receiver.accept(&disagreeing, t0);

    assert!(deliveries(&mut receiver, t0).is_empty());

    feed(&mut receiver, &datagrams[3], t0);

    let delivered = deliveries(&mut receiver, t0);
    assert_eq!(delivered.len(), 1);
    assert_eq!(delivered[0].payload, payload);
}
