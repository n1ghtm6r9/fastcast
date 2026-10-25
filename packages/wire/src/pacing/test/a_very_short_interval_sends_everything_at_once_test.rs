use crate::pacing::burst_sizes;
use std::time::Duration;

#[test]
fn a_very_short_interval_sends_everything_at_once() {
    assert_eq!(burst_sizes(50, Duration::from_micros(600)), vec![50]);
}
