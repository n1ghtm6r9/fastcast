use crate::link::test::{ReceivedPacket, opened_link};
use std::time::Duration;

#[test]
fn notices_before_the_welcome_are_refused() {
    let (server, link) = opened_link();

    assert!(!link.send_notice(b"{\"type\":\"scale\",\"step\":1}"));

    let sent = server
        .packets_within(Duration::from_millis(100))
        .into_iter()
        .any(|packet| matches!(packet, ReceivedPacket::Notice { .. }));
    assert!(!sent, "a Notice datagram left before the Welcome");
}
