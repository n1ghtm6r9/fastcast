use crate::pacing::burst_sizes;
use std::time::Duration;

#[test]
fn no_packets_means_no_bursts() {
    assert_eq!(
        burst_sizes(0, Duration::from_micros(8333)),
        Vec::<usize>::new()
    );
}
