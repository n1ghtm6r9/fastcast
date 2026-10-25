use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, feed, ms};
use std::time::Instant;

#[test]
fn next_deadline_is_the_frame_timeout_after_the_first_shard() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();
    let datagrams = datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 3000], 10);

    feed(&mut receiver, &datagrams[0], t0);

    assert_eq!(receiver.next_deadline(), Some(t0 + ms(100)));
}
