use crate::packet::{Datagram, Packet, decode_datagram};
use crate::packetizer::test::video_frame;
use crate::packetizer::{Datagrams, Packetizer};

#[test]
fn control_packets_take_the_next_sequence() {
    let payload = vec![0u8; 100];
    let mut packetizer = Packetizer::new(7);
    let mut out = Datagrams::new();
    let mut bytes = Vec::new();

    packetizer.frame(&video_frame(&payload, true), 10, &mut out);
    packetizer.control(&Packet::Bye, &mut bytes);

    assert_eq!(
        decode_datagram(&bytes),
        Some(Datagram {
            session: 7,
            sequence: 2,
            packet: Packet::Bye
        })
    );
}
