use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed, ms};
use std::time::Instant;

#[test]
fn a_complete_later_frame_waits_the_reorder_window() {
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

    assert!(deliveries(&mut receiver, t0 + ms(4)).is_empty());
    assert_eq!(receiver.lost_frames(), 0);
    assert!(deliveries(&mut receiver, t0 + ms(5)).is_empty());
    assert_eq!(receiver.lost_frames(), 1);
}
