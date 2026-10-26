use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed, ms};
use std::time::Instant;

#[test]
fn video_after_a_loss_waits_for_the_next_keyframe() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }
    assert_eq!(deliveries(&mut receiver, t0).len(), 1);

    datagrams_of(&mut packetizer, FrameKind::Video, false, &[2; 100], 10);
    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, false, &[3; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }
    assert!(deliveries(&mut receiver, t0 + ms(5)).is_empty());
    assert!(receiver.keyframe_needed());

    for datagram in datagrams_of(&mut packetizer, FrameKind::Config, false, &[7; 100], 10) {
        feed(&mut receiver, &datagram, t0 + ms(5));
    }
    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &[4; 100], 10) {
        feed(&mut receiver, &datagram, t0 + ms(5));
    }

    let numbers: Vec<u32> = deliveries(&mut receiver, t0 + ms(5))
        .iter()
        .map(|frame| frame.number)
        .collect();
    assert_eq!(numbers, [3, 4]);
    assert!(!receiver.keyframe_needed());
}
