use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use crate::receiver::{FrameReceiver, ReceivedFrame};
use std::time::Instant;

#[test]
fn a_frame_arrives_whole_when_every_datagram_does() {
    let payload: Vec<u8> = (0..5000).map(|index| (index % 251) as u8).collect();
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &payload, 10) {
        feed(&mut receiver, &datagram, t0);
    }

    assert_eq!(
        deliveries(&mut receiver, t0),
        vec![ReceivedFrame {
            number: 0,
            kind: FrameKind::Video,
            keyframe: true,
            payload
        }]
    );
}
