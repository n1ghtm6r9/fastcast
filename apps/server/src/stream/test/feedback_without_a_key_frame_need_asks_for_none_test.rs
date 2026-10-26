use super::feedback_of;
use crate::stream::SessionControl;

#[test]
fn feedback_without_a_key_frame_need_asks_for_none() {
    let control = SessionControl::new();

    control.report(&feedback_of(5, false), 10);

    assert!(!control.take_key_frame_need(None));
}
