use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn a_picture_arriving_at_twice_the_frame_rate_waits_for_its_turn() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded_picture(start, start);

    assert!(!clock.may_encode(start + Duration::from_millis(5)));
}
