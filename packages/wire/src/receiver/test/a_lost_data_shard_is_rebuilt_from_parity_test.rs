use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use crate::test_support::random_bytes;
use std::time::Instant;

#[test]
fn a_lost_data_shard_is_rebuilt_from_parity() {
    let mut state = 21;
    let payload = random_bytes(&mut state, 10_000);
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &payload, 20)
        .iter()
        .skip(1)
    {
        feed(&mut receiver, datagram, t0);
    }

    let delivered = deliveries(&mut receiver, t0);
    assert_eq!(delivered.len(), 1);
    assert_eq!(delivered[0].payload, payload);
    assert_eq!(receiver.recovered_shards(), 1);
}
