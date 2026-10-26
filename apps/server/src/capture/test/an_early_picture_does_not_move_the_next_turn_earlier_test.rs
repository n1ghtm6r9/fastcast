use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn an_early_picture_does_not_move_the_next_turn_earlier() {
    let start = Instant::now();
    let mut clock = FrameClock::new(100);

    clock.encoded(start);
    clock.encoded(start + Duration::from_millis(8));

    assert_eq!(
        clock.until_repeat(start + Duration::from_millis(8)),
        Duration::from_millis(12)
    );
}
