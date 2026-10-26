use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed, ms};
use std::time::Instant;

#[test]
fn a_key_frame_after_a_lost_config_still_asks_for_one() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    datagrams_of(&mut packetizer, FrameKind::Config, false, &[7; 100], 10);
    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }

    let numbers: Vec<u32> = deliveries(&mut receiver, t0 + ms(100))
        .iter()
        .map(|frame| frame.number)
        .collect();
    assert_eq!(numbers, [1]);
    assert_eq!(receiver.lost_frames(), 1);
    assert!(receiver.keyframe_needed());
}
