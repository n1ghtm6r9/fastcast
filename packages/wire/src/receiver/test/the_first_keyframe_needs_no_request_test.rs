use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use std::time::Instant;

#[test]
fn the_first_keyframe_needs_no_request() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for datagram in datagrams_of(&mut packetizer, FrameKind::Config, false, &[7; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }

    assert_eq!(deliveries(&mut receiver, t0).len(), 1);
    assert!(!receiver.keyframe_needed());
}
