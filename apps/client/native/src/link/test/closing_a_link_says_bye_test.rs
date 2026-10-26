use crate::link::test::{ReceivedPacket, TEST_SESSION, connected_link};
use std::time::Duration;

#[test]
fn closing_a_link_says_bye() {
    let (server, link, _peer) = connected_link();

    link.close();

    assert_eq!(server.expect_bye(), TEST_SESSION);

    link.close();

    let byes = server
        .packets_within(Duration::from_millis(150))
        .into_iter()
        .filter(|packet| matches!(packet, ReceivedPacket::Bye { .. }))
        .count();
    assert_eq!(byes, 0);
}
