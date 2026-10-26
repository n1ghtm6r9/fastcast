use crate::link::LinkEvent;
use crate::link::test::{TEST_SESSION, TEST_WAIT, connected_link, packetized_frame};
use std::net::UdpSocket;
use std::time::Duration;
use wire::{FrameKind, Packetizer};

#[test]
fn datagrams_from_another_address_are_ignored() {
    let (server, link, peer) = connected_link();
    let stranger = UdpSocket::bind("127.0.0.1:0").unwrap();
    let mut packetizer = Packetizer::new(TEST_SESSION);
    let datagrams = packetized_frame(&mut packetizer, FrameKind::Video, true, &[1, 2, 3], 0);

    for datagram in &datagrams {
        stranger.send_to(datagram, peer).unwrap();
    }

    assert_eq!(link.next_event(Duration::from_millis(300)), None);

    server.send_datagrams(peer, &datagrams);

    assert_eq!(
        link.next_event(TEST_WAIT),
        Some(LinkEvent::Video(vec![1, 2, 3]))
    );
}
