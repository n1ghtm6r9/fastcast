use super::feedback_of;
use crate::stream::SessionControl;

#[test]
fn feedback_sets_the_fec_level_the_sender_uses() {
    let control = SessionControl::new();

    control.report(&feedback_of(5, false), 27);

    assert_eq!(control.fec_percent(), 27);
}
