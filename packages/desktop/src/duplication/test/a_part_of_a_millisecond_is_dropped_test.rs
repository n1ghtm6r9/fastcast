use crate::duplication::timeout_milliseconds;
use std::time::Duration;

#[test]
fn a_part_of_a_millisecond_is_dropped() {
    assert_eq!(timeout_milliseconds(Duration::from_micros(1_999)), 1);
    assert_eq!(timeout_milliseconds(Duration::from_micros(999)), 0);
}
