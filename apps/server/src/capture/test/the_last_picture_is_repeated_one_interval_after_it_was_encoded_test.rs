use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn the_last_picture_is_repeated_one_interval_after_it_was_encoded() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded(start);

    assert_eq!(
        clock.until_repeat(start + Duration::from_millis(4)),
        Duration::from_millis(6)
    );
    assert_eq!(
        clock.until_repeat(start + Duration::from_millis(12)),
        Duration::ZERO
    );
}
