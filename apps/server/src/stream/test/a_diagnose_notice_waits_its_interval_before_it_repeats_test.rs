use crate::stream::{DIAGNOSE_NOTICE_INTERVAL, DiagnoseRepeat};
use std::time::{Duration, Instant};

#[test]
fn a_diagnose_notice_waits_its_interval_before_it_repeats() {
    let start = Instant::now();
    let mut repeat = DiagnoseRepeat::new(Some(7));

    assert_eq!(repeat.due(start), Some(7));
    assert_eq!(repeat.due(start), None);
    assert_eq!(
        repeat.due(start + DIAGNOSE_NOTICE_INTERVAL - Duration::from_millis(1)),
        None
    );
    assert_eq!(repeat.due(start + DIAGNOSE_NOTICE_INTERVAL), Some(7));
    assert_eq!(repeat.due(start + DIAGNOSE_NOTICE_INTERVAL), None);
}
