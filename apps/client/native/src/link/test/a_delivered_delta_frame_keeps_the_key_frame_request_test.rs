use crate::link::test::{HandDrivenLink, TEST_SESSION, packetized_frame};
use std::time::Duration;
use wire::{FrameKind, Packetizer};

#[test]
fn a_delivered_delta_frame_keeps_the_key_frame_request() {
    let mut link = HandDrivenLink::new();
    link.welcome();
    let mut packetizer = Packetizer::new(TEST_SESSION);
    let key_frame = packetized_frame(&mut packetizer, FrameKind::Video, true, &[1; 100], 0);
    let delta_frame = packetized_frame(&mut packetizer, FrameKind::Video, false, &[2; 100], 0);

    link.feed(&key_frame, Duration::ZERO);
    link.deliver_at(Duration::ZERO);
    link.shared.request_key_frame();
    link.feed(&delta_frame, Duration::from_millis(5));
    link.deliver_at(Duration::from_millis(5));
    link.feedback_at(Duration::from_millis(20));

    assert!(link.server.expect_feedback().keyframe_needed);
}
