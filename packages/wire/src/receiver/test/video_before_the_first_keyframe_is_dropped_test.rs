use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use std::time::Instant;

#[test]
fn video_before_the_first_keyframe_is_dropped() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, false, &[1; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }

    assert!(deliveries(&mut receiver, t0).is_empty());
    assert!(receiver.keyframe_needed());
}
