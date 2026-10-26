use crate::link::test::HandDrivenLink;
use std::time::Duration;

#[test]
fn a_key_frame_request_is_added_to_the_feedback() {
    let mut link = HandDrivenLink::new();
    link.welcome();

    link.feedback_at(Duration::ZERO);
    assert!(!link.server.expect_feedback().keyframe_needed);

    link.shared.request_key_frame();
    link.feedback_at(Duration::from_millis(20));

    assert!(link.server.expect_feedback().keyframe_needed);
}
