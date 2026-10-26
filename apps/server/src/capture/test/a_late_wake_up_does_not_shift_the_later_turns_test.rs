use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn a_late_wake_up_does_not_shift_the_later_turns() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded(start);
    clock.encoded(start + Duration::from_millis(13));

    assert_eq!(
        clock.until_repeat(start + Duration::from_millis(13)),
        Duration::from_millis(7)
    );
}
