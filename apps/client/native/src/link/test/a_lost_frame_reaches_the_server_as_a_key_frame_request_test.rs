use crate::link::LinkEvent;
use crate::link::test::{TEST_WAIT, connected_link};
use wire::{FrameKind, SHARD_SIZE};

#[test]
fn a_lost_frame_reaches_the_server_as_a_key_frame_request() {
    let (mut server, link, peer) = connected_link();
    let key_frame = vec![1u8; 100];

    server.send_frame(peer, FrameKind::Video, true, &key_frame, 0);
    assert_eq!(
        link.next_event(TEST_WAIT),
        Some(LinkEvent::Video(key_frame))
    );

    let damaged = server.packetize_frame(FrameKind::Video, false, &vec![2u8; 3 * SHARD_SIZE], 0);
    server.send_datagrams(peer, &damaged[1..]);
    server.send_frame(peer, FrameKind::Video, false, &[3u8; 100], 0);

    server.expect_feedback_where(|feedback| feedback.keyframe_needed && feedback.lost_frames == 1);
}
