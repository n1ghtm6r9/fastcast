use crate::link::test::{ReceivedPacket, opened_link};
use std::time::Duration;

#[test]
fn stats_before_the_welcome_are_refused() {
    let (server, link) = opened_link();

    assert!(!link.send_stats(b"{\"type\":\"stats\"}"));

    let sent = server
        .packets_within(Duration::from_millis(100))
        .into_iter()
        .any(|packet| matches!(packet, ReceivedPacket::Stats { .. }));
    assert!(!sent, "a Stats datagram left before the Welcome");
}
