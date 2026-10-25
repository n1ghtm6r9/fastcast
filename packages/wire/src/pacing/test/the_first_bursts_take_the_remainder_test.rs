use crate::pacing::burst_sizes;
use std::time::Duration;

#[test]
fn the_first_bursts_take_the_remainder() {
    assert_eq!(
        burst_sizes(10, Duration::from_micros(8333)),
        vec![2, 2, 1, 1, 1, 1, 1, 1]
    );
}
