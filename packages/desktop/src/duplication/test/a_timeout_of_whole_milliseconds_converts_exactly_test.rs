use crate::duplication::timeout_milliseconds;
use std::time::Duration;

#[test]
fn a_timeout_of_whole_milliseconds_converts_exactly() {
    assert_eq!(timeout_milliseconds(Duration::from_millis(250)), 250);
}
