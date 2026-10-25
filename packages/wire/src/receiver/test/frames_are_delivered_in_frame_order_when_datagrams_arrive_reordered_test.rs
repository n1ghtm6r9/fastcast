use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use std::time::Instant;

#[test]
fn frames_are_delivered_in_frame_order_when_datagrams_arrive_reordered() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();
    let first = datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 3000], 10);
    let second = datagrams_of(&mut packetizer, FrameKind::Video, false, &[2; 3000], 10);

    for datagram in second.iter().chain(&first) {
        feed(&mut receiver, datagram, t0);
    }

    let numbers: Vec<u32> = deliveries(&mut receiver, t0)
        .iter()
        .map(|frame| frame.number)
        .collect();
    assert_eq!(numbers, [0, 1]);
}
