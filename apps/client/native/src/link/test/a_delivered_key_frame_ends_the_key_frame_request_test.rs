use crate::link::test::{HandDrivenLink, TEST_SESSION, packetized_frame};
use std::time::Duration;
use wire::{FrameKind, Packetizer};

#[test]
fn a_delivered_key_frame_ends_the_key_frame_request() {
    let mut link = HandDrivenLink::new();
    link.welcome();
    let mut packetizer = Packetizer::new(TEST_SESSION);
    let key_frame = packetized_frame(&mut packetizer, FrameKind::Video, true, &[1; 100], 0);

    link.shared.request_key_frame();
    link.feedback_at(Duration::ZERO);
    assert!(link.server.expect_feedback().keyframe_needed);

    link.feed(&key_frame, Duration::from_millis(5));
    link.deliver_at(Duration::from_millis(5));
    link.feedback_at(Duration::from_millis(20));

    assert!(!link.server.expect_feedback().keyframe_needed);
}
