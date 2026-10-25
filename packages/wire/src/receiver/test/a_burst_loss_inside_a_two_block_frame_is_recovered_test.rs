use crate::packet::{FrameKind, SHARD_SIZE};
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use crate::test_support::random_bytes;
use std::time::Instant;

#[test]
fn a_burst_loss_inside_a_two_block_frame_is_recovered() {
    let mut state = 23;
    let payload = random_bytes(&mut state, 200 * SHARD_SIZE);
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for (index, datagram) in datagrams_of(&mut packetizer, FrameKind::Video, true, &payload, 10)
        .iter()
        .enumerate()
    {
        if !(10..30).contains(&index) {
            feed(&mut receiver, datagram, t0);
        }
    }

    let delivered = deliveries(&mut receiver, t0);
    assert_eq!(delivered.len(), 1);
    assert_eq!(delivered[0].payload, payload);
    assert_eq!(receiver.recovered_shards(), 20);
}
