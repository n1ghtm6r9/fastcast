use crate::duplication::timeout_milliseconds;
use std::time::Duration;

#[test]
fn a_part_of_a_millisecond_rounds_up() {
    assert_eq!(timeout_milliseconds(Duration::from_micros(1_001)), 2);
}
