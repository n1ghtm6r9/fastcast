use crate::pacing::burst_sizes;
use std::time::Duration;

#[test]
fn a_frame_is_spread_over_half_the_frame_interval() {
    assert_eq!(burst_sizes(80, Duration::from_micros(8333)), vec![10; 8]);
}
