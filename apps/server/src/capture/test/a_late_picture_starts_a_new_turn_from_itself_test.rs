use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn a_late_picture_starts_a_new_turn_from_itself() {
    let start = Instant::now();
    let late = start + Duration::from_millis(25);
    let mut clock = FrameClock::new(100);

    clock.encoded_picture(start, start);
    clock.encoded_picture(late, late);

    assert_eq!(clock.until_repeat(late), Duration::from_micros(12_500));
}
