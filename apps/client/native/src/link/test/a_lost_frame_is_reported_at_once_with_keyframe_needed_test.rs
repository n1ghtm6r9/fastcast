use crate::link::LinkEvent;
use crate::link::test::{ReceivedPacket, TEST_SESSION, TEST_WAIT, connected_link};
use std::time::{Duration, Instant};
use wire::{FrameKind, SHARD_SIZE};

#[test]
fn a_lost_frame_is_reported_at_once_with_keyframe_needed() {
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

    let deadline = Instant::now() + Duration::from_millis(150);
    let mut reported = false;

    while let Some(remaining) = deadline.checked_duration_since(Instant::now()) {
        if let Some((
            ReceivedPacket::Feedback {
                session: TEST_SESSION,
                feedback,
            },
            _,
        )) = server.next_packet(remaining)
            && feedback.keyframe_needed
            && feedback.lost_frames == 1
        {
            reported = true;
            break;
        }
    }

    assert!(reported, "no loss report within 150 ms of the next frame");
}
