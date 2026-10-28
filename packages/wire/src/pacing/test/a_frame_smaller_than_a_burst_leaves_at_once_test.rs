use crate::pacing::burst_sizes;
use std::time::Duration;

#[test]
fn a_frame_smaller_than_a_burst_leaves_at_once() {
    assert_eq!(burst_sizes(10, Duration::from_micros(16667)), vec![10]);
}
