use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use std::time::Instant;

#[test]
fn more_than_64_pending_frames_drop_the_oldest() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }
    assert_eq!(deliveries(&mut receiver, t0).len(), 1);

    for _ in 1..=65 {
        let datagrams = datagrams_of(&mut packetizer, FrameKind::Video, false, &[2; 3000], 0);
        feed(&mut receiver, &datagrams[0], t0);
    }

    assert_eq!(receiver.lost_frames(), 1);
}
