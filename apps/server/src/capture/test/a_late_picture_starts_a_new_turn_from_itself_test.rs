use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn a_late_picture_starts_a_new_turn_from_itself() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded(start);
    clock.encoded(start + Duration::from_millis(25));

    assert_eq!(
        clock.until_repeat(start + Duration::from_millis(25)),
        Duration::from_millis(10)
    );
}
