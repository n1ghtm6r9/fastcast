use crate::packet::{FrameKind, SHARD_SIZE};
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed, ms};
use std::time::Instant;

#[test]
fn the_byte_budget_is_released_when_frames_leave() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();
    let first = datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 3000], 10);
    let second = datagrams_of(&mut packetizer, FrameKind::Video, true, &[2; 3000], 10);
    let block_bytes = (3 + 1) * SHARD_SIZE;

    feed(&mut receiver, &first[0], t0);
    assert_eq!(receiver.pending_bytes, block_bytes);

    for datagram in &first[1..] {
        feed(&mut receiver, datagram, t0);
    }
    assert_eq!(deliveries(&mut receiver, t0).len(), 1);
    assert_eq!(receiver.pending_bytes, 0);

    feed(&mut receiver, &second[0], t0);
    assert_eq!(receiver.pending_bytes, block_bytes);

    assert!(deliveries(&mut receiver, t0 + ms(100)).is_empty());
    assert_eq!(receiver.lost_frames(), 1);
    assert_eq!(receiver.pending_bytes, 0);
}
