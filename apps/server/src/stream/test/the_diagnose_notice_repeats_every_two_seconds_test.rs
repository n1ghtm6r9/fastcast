use crate::stream::DIAGNOSE_NOTICE_INTERVAL;
use std::time::Duration;

#[test]
fn the_diagnose_notice_repeats_every_two_seconds() {
    assert_eq!(DIAGNOSE_NOTICE_INTERVAL, Duration::from_secs(2));
}
