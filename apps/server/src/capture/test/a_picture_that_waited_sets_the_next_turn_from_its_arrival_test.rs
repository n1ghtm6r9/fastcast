use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn a_picture_that_waited_sets_the_next_turn_from_its_arrival() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded_picture(start, start);
    clock.encoded_picture(
        start + Duration::from_millis(4),
        start + Duration::from_micros(12_500),
    );

    assert_eq!(
        clock.until_repeat(start + Duration::from_micros(12_500)),
        Duration::from_millis(4)
    );
    assert!(clock.may_encode(start + Duration::from_millis(14)));
}
