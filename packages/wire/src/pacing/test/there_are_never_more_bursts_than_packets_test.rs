use crate::pacing::burst_sizes;
use std::time::Duration;

#[test]
fn there_are_never_more_bursts_than_packets() {
    assert_eq!(burst_sizes(3, Duration::from_micros(8333)), vec![1, 1, 1]);
}
