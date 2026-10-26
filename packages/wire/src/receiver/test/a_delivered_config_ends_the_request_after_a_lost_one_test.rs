use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed, ms};
use std::time::Instant;

#[test]
fn a_delivered_config_ends_the_request_after_a_lost_one() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    datagrams_of(&mut packetizer, FrameKind::Config, false, &[7; 100], 10);
    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }
    assert_eq!(deliveries(&mut receiver, t0 + ms(100)).len(), 1);
    assert!(receiver.keyframe_needed());

    for datagram in datagrams_of(&mut packetizer, FrameKind::Config, false, &[7; 100], 10) {
        feed(&mut receiver, &datagram, t0 + ms(100));
    }
    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &[2; 100], 10) {
        feed(&mut receiver, &datagram, t0 + ms(100));
    }

    let numbers: Vec<u32> = deliveries(&mut receiver, t0 + ms(100))
        .iter()
        .map(|frame| frame.number)
        .collect();
    assert_eq!(numbers, [2, 3]);
    assert!(!receiver.keyframe_needed());
}
