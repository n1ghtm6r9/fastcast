use crate::stream::frame_interval_of;
use std::time::Duration;

#[test]
fn a_rate_of_zero_falls_back_to_sixty() {
    assert_eq!(frame_interval_of(0), Duration::from_secs(1) / 60);
}
