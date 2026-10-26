use crate::stream::frame_interval_of;
use std::time::Duration;

#[test]
fn the_frame_interval_is_one_second_over_the_rate() {
    assert_eq!(frame_interval_of(60), Duration::from_nanos(16_666_666));
    assert_eq!(frame_interval_of(60), Duration::from_secs(1) / 60);
    assert_eq!(frame_interval_of(120), Duration::from_secs(1) / 120);
}
