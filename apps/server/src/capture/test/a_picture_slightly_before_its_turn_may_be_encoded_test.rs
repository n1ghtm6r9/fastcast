use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn a_picture_slightly_before_its_turn_may_be_encoded() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded(start);

    assert!(clock.may_encode(start + Duration::from_micros(7_500)));
}
