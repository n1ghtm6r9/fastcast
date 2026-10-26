use crate::link::test::{ReceivedPacket, TEST_SESSION, connected_link};
use std::time::Duration;

#[test]
fn feedback_flows_every_twenty_milliseconds() {
    let (server, _link, _peer) = connected_link();

    let feedbacks = server
        .packets_within(Duration::from_millis(250))
        .into_iter()
        .filter(|packet| {
            matches!(
                packet,
                ReceivedPacket::Feedback {
                    session: TEST_SESSION,
                    ..
                }
            )
        })
        .count();

    assert!(
        feedbacks >= 8,
        "only {feedbacks} Feedback datagrams in 250 ms"
    );
}
