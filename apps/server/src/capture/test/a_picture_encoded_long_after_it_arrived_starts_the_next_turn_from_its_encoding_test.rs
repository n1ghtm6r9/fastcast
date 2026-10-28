use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn a_picture_encoded_long_after_it_arrived_starts_the_next_turn_from_its_encoding() {
    let start = Instant::now();
    let encoded = start + Duration::from_millis(25);
    let mut clock = FrameClock::new(100);

    clock.encoded_picture(start, start);
    clock.encoded_picture(start + Duration::from_millis(1), encoded);

    assert_eq!(clock.until_repeat(encoded), Duration::from_micros(12_500));
}
