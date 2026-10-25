use crate::packet::DATAGRAM_LIMIT;
use crate::packetizer::test::video_frame;
use crate::packetizer::{Datagrams, Packetizer};
use crate::test_support::random_bytes;

#[test]
fn no_datagram_exceeds_the_datagram_limit() {
    let mut state = 3;
    let payload = random_bytes(&mut state, 500_000);
    let mut packetizer = Packetizer::new(1);
    let mut out = Datagrams::new();

    packetizer.frame(&video_frame(&payload, true), 50, &mut out);

    assert!(!out.is_empty());
    assert!(out.iter().all(|datagram| datagram.len() <= DATAGRAM_LIMIT));
}
