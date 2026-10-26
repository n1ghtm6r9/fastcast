use super::feedback_of;
use crate::stream::SessionControl;

#[test]
fn a_key_frame_need_reported_before_the_last_key_frame_arrived_is_ignored() {
    let control = SessionControl::new();

    control.report(&feedback_of(40, true), 10);

    assert!(!control.take_key_frame_need(Some(41)));
    assert!(!control.take_key_frame_need(None));
}
