use crate::capture::FrameClock;
use std::time::{Duration, Instant};

#[test]
fn before_any_picture_the_wait_is_one_interval() {
    assert_eq!(
        FrameClock::new(100).until_repeat(Instant::now()),
        Duration::from_millis(10)
    );
}
