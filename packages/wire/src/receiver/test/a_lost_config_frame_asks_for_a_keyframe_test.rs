use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed, ms};
use std::time::Instant;

#[test]
fn a_lost_config_frame_asks_for_a_keyframe() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();
    let datagrams = datagrams_of(&mut packetizer, FrameKind::Config, false, &[3; 3000], 0);

    feed(&mut receiver, &datagrams[0], t0);

    assert!(deliveries(&mut receiver, t0 + ms(100)).is_empty());
    assert_eq!(receiver.lost_frames(), 1);
    assert!(receiver.keyframe_needed());
}
