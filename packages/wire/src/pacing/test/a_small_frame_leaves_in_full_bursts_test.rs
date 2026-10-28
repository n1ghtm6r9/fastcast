use crate::pacing::burst_sizes;
use std::time::Duration;

#[test]
fn a_small_frame_leaves_in_full_bursts() {
    assert_eq!(burst_sizes(48, Duration::from_micros(8333)), vec![24, 24]);
}
