use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn a_repeat_does_not_hold_back_the_next_picture() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded_picture(start, start);
    clock.encoded(start + Duration::from_micros(12_500));

    assert!(clock.may_encode(start + Duration::from_millis(13)));
}
