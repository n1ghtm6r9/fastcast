use crate::link::test::{ReceivedPacket, TEST_WAIT, connected_link};
use crate::link::{CLOSED_BY_SERVER, LinkEvent};
use std::time::{Duration, Instant};

#[test]
fn a_bye_from_the_server_ends_the_link() {
    let (mut server, link, peer) = connected_link();

    server.send_bye(peer);

    assert_eq!(
        link.next_event(TEST_WAIT),
        Some(LinkEvent::Ended(CLOSED_BY_SERVER.to_string()))
    );

    let asked_at = Instant::now();
    assert_eq!(link.next_event(TEST_WAIT), None);
    assert!(asked_at.elapsed() < Duration::from_millis(500));

    link.close();

    let answered = server
        .packets_within(Duration::from_millis(150))
        .into_iter()
        .any(|packet| matches!(packet, ReceivedPacket::Bye { .. }));
    assert!(!answered, "the link answered the server's Bye");
}
