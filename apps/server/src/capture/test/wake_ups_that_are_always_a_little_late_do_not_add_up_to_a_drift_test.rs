use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn wake_ups_that_are_always_a_little_late_do_not_add_up_to_a_drift() {
    let start = Instant::now();
    let late = Duration::from_millis(3);
    let mut clock = FrameClock::new(100);

    clock.encoded(start);

    for turn in 1..=50 {
        clock.encoded(start + Duration::from_millis(10) * turn + late);
    }

    assert_eq!(
        clock.until_repeat(start + Duration::from_millis(500) + late),
        Duration::from_micros(9_500)
    );
}
