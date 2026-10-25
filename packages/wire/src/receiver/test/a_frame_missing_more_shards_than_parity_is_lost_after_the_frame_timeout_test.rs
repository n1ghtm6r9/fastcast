use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed, ms};
use std::time::Instant;

#[test]
fn a_frame_missing_more_shards_than_parity_is_lost_after_the_frame_timeout() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }
    assert_eq!(deliveries(&mut receiver, t0).len(), 1);

    let second = datagrams_of(&mut packetizer, FrameKind::Video, false, &[2; 3000], 10);
    for datagram in &second[2..] {
        feed(&mut receiver, datagram, t0);
    }

    assert!(deliveries(&mut receiver, t0 + ms(99)).is_empty());
    assert_eq!(receiver.lost_frames(), 0);
    assert!(deliveries(&mut receiver, t0 + ms(100)).is_empty());
    assert_eq!(receiver.lost_frames(), 1);
    assert!(receiver.keyframe_needed());
}
