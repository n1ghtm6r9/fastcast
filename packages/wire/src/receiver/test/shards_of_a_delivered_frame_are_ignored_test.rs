use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use std::time::Instant;

#[test]
fn shards_of_a_delivered_frame_are_ignored() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();
    let first = datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 3000], 10);
    let second = datagrams_of(&mut packetizer, FrameKind::Video, false, &[2; 3000], 10);

    for datagram in first.iter().chain(&second) {
        feed(&mut receiver, datagram, t0);
    }
    assert_eq!(deliveries(&mut receiver, t0).len(), 2);

    for datagram in first.iter().chain(&second) {
        feed(&mut receiver, datagram, t0);
    }

    assert!(deliveries(&mut receiver, t0).is_empty());
    assert_eq!(receiver.highest_frame(), 1);
    assert_eq!(receiver.next_deadline(), None);
}
