use crate::link::test::{ReceivedPacket, connected_link};
use std::time::Duration;

#[test]
fn stats_longer_than_a_datagram_are_refused() {
    let (server, link, _peer) = connected_link();

    assert!(!link.send_stats(&[b'a'; 1389]));
    assert!(link.send_stats(&[b'b'; 1388]));

    let texts: Vec<String> = server
        .packets_within(Duration::from_millis(200))
        .into_iter()
        .filter_map(|packet| match packet {
            ReceivedPacket::Stats { text, .. } => Some(text),
            _ => None,
        })
        .collect();
    assert_eq!(texts, vec!["b".repeat(1388)]);
}
