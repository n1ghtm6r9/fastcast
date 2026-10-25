use crate::layout::FrameLayout;
use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use crate::test_support::random_bytes;
use std::time::Instant;

#[test]
fn a_multi_megabyte_keyframe_with_uneven_blocks_survives_a_burst_loss() {
    let mut state = 41;
    let payload = random_bytes(&mut state, 3_000_000);
    let layout = FrameLayout::new(payload.len() as u32);
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    assert_ne!(layout.data_shards % layout.blocks, 0);

    for (index, datagram) in datagrams_of(&mut packetizer, FrameKind::Video, true, &payload, 10)
        .iter()
        .enumerate()
    {
        if !(100..160).contains(&index) {
            feed(&mut receiver, datagram, t0);
        }
    }

    let delivered = deliveries(&mut receiver, t0);
    assert_eq!(delivered.len(), 1);
    assert_eq!(delivered[0].payload, payload);
    assert_eq!(receiver.recovered_shards(), 60);
}
