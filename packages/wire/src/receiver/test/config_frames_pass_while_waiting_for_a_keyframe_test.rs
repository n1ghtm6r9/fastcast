use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use crate::receiver::{FrameReceiver, ReceivedFrame};
use std::time::Instant;

#[test]
fn config_frames_pass_while_waiting_for_a_keyframe() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for datagram in datagrams_of(&mut packetizer, FrameKind::Config, false, &[7; 100], 10) {
        feed(&mut receiver, &datagram, t0);
    }

    assert_eq!(
        deliveries(&mut receiver, t0),
        vec![ReceivedFrame {
            number: 0,
            kind: FrameKind::Config,
            keyframe: false,
            payload: vec![7; 100]
        }]
    );
}
