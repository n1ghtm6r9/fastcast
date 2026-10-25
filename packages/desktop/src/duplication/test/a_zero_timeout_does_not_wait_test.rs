use crate::duplication::timeout_milliseconds;
use std::time::Duration;

#[test]
fn a_zero_timeout_does_not_wait() {
    assert_eq!(timeout_milliseconds(Duration::ZERO), 0);
}
