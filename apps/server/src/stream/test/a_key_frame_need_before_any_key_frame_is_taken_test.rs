use super::feedback_of;
use crate::stream::SessionControl;

#[test]
fn a_key_frame_need_before_any_key_frame_is_taken() {
    let control = SessionControl::new();

    control.report(&feedback_of(0, true), 10);

    assert!(control.take_key_frame_need(None));
    assert!(!control.take_key_frame_need(None));
}
