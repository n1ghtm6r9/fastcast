use crate::duplication::timeout_milliseconds;
use std::time::Duration;

#[test]
fn a_very_long_timeout_saturates() {
    assert_eq!(
        timeout_milliseconds(Duration::from_secs(u64::MAX)),
        u32::MAX
    );
}
