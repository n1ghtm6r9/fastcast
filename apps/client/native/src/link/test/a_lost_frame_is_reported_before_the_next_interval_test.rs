use crate::link::test::{HandDrivenLink, TEST_SESSION, packetized_frame};
use std::time::Duration;
use wire::{FrameKind, Packetizer, SHARD_SIZE};

#[test]
fn a_lost_frame_is_reported_before_the_next_interval() {
    let mut link = HandDrivenLink::new();
    link.welcome();
    let mut packetizer = Packetizer::new(TEST_SESSION);
    let key_frame = packetized_frame(&mut packetizer, FrameKind::Video, true, &[1; 100], 0);
    let damaged = packetized_frame(
        &mut packetizer,
        FrameKind::Video,
        false,
        &vec![2; 3 * SHARD_SIZE],
        0,
    );
    let next_frame = packetized_frame(&mut packetizer, FrameKind::Video, false, &[3; 100], 0);

    link.feed(&key_frame, Duration::ZERO);
    link.deliver_at(Duration::ZERO);
    link.feedback_at(Duration::ZERO);
    assert_eq!(link.server.expect_feedback().lost_frames, 0);

    link.feed(&damaged[1..], Duration::from_millis(1));
    link.feed(&next_frame, Duration::from_millis(1));
    link.deliver_at(Duration::from_millis(10));
    link.feedback_at(Duration::from_millis(10));

    let feedback = link.server.expect_feedback();
    assert_eq!(feedback.lost_frames, 1);
    assert!(feedback.keyframe_needed);
}
