use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn a_picture_arriving_just_after_its_turn_is_waited_for() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded(start);

    assert_eq!(
        clock.until_repeat(start + Duration::from_millis(11)),
        Duration::from_micros(1_500)
    );
    assert!(clock.may_encode(start + Duration::from_millis(11)));
}
