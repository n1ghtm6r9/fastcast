use crate::stream::SessionControl;
use std::time::Duration;

#[test]
fn taking_the_send_times_starts_counting_afresh() {
    let control = SessionControl::new();
    control.sent(Duration::from_millis(4));

    assert_eq!(control.take_sent(), vec![Duration::from_millis(4)]);
    assert!(control.take_sent().is_empty());
}
