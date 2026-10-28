use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn an_early_picture_sets_the_next_turn_one_interval_after_it() {
    let start = Instant::now();
    let early = start + Duration::from_millis(8);
    let mut clock = FrameClock::new(100);

    clock.encoded_picture(start, start);
    clock.encoded_picture(early, early);

    assert_eq!(clock.until_repeat(early), Duration::from_micros(12_500));
}
