use crate::stream::SessionControl;

#[test]
fn a_requested_welcome_is_taken_once() {
    let control = SessionControl::new();

    control.want_welcome();

    assert!(control.take_welcome());
    assert!(!control.take_welcome());
}
