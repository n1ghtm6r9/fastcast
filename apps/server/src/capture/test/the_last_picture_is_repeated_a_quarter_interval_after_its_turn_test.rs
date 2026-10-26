use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn the_last_picture_is_repeated_a_quarter_interval_after_its_turn() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded(start);

    assert_eq!(
        clock.until_repeat(start + Duration::from_millis(4)),
        Duration::from_micros(8_500)
    );
    assert_eq!(
        clock.until_repeat(start + Duration::from_millis(12)),
        Duration::from_micros(500)
    );
    assert_eq!(
        clock.until_repeat(start + Duration::from_micros(12_500)),
        Duration::ZERO
    );
}
