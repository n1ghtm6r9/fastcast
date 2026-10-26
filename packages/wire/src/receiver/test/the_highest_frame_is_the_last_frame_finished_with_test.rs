use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use crate::receiver::{FRAME_TIMEOUT, FrameReceiver};
use std::time::Instant;

#[test]
fn the_highest_frame_is_the_last_frame_finished_with() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for datagram in datagrams_of(&mut packetizer, FrameKind::Config, false, &[7; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }
    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }
    assert_eq!(deliveries(&mut receiver, t0).len(), 2);

    let third = datagrams_of(&mut packetizer, FrameKind::Video, true, &[2; 3000], 10);
    feed(&mut receiver, &third[0], t0);

    assert!(deliveries(&mut receiver, t0).is_empty());
    assert_eq!(receiver.highest_frame(), 1);

    assert!(deliveries(&mut receiver, t0 + FRAME_TIMEOUT).is_empty());
    assert_eq!(receiver.lost_frames(), 1);
    assert_eq!(receiver.highest_frame(), 2);
}
