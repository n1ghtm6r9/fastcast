use crate::link::test::{ReceivedPacket, connected_link};
use std::time::Duration;

#[test]
fn stats_that_are_not_text_are_refused() {
    let (server, link, _peer) = connected_link();

    assert!(!link.send_stats(&[0xff, 0xfe, 0xfd]));

    let sent = server
        .packets_within(Duration::from_millis(100))
        .into_iter()
        .any(|packet| matches!(packet, ReceivedPacket::Stats { .. }));
    assert!(!sent, "bytes that are not UTF-8 left as Stats");
}
