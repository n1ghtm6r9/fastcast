use crate::link::LinkEvent;
use crate::link::test::{TEST_SESSION, TEST_WAIT, connected_link, packetized_frame};
use std::time::Duration;
use wire::{FrameKind, Packetizer};

#[test]
fn shards_of_another_session_are_ignored() {
    let (mut server, link, peer) = connected_link();
    let mut other_session = Packetizer::new(TEST_SESSION + 1);
    let datagrams = packetized_frame(&mut other_session, FrameKind::Video, true, &[9, 9], 0);

    server.send_datagrams(peer, &datagrams);

    assert_eq!(link.next_event(Duration::from_millis(300)), None);

    server.send_frame(peer, FrameKind::Video, true, &[1, 2, 3], 0);

    assert_eq!(
        link.next_event(TEST_WAIT),
        Some(LinkEvent::Video(vec![1, 2, 3]))
    );
}
