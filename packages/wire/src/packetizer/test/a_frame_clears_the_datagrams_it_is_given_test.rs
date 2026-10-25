use crate::packetizer::test::video_frame;
use crate::packetizer::{Datagrams, Packetizer};

#[test]
fn a_frame_clears_the_datagrams_it_is_given() {
    let payload = vec![0u8; 100];
    let mut packetizer = Packetizer::new(1);
    let mut out = Datagrams::new();

    packetizer.frame(&video_frame(&payload, true), 10, &mut out);
    packetizer.frame(&video_frame(&payload, false), 10, &mut out);

    assert_eq!(out.len(), 2);
}
