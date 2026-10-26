use super::feedback_of;
use crate::stream::SessionControl;

#[test]
fn a_key_frame_need_after_the_last_key_frame_is_taken() {
    let control = SessionControl::new();

    control.report(&feedback_of(41, true), 10);

    assert!(control.take_key_frame_need(Some(41)));
}
