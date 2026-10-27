use crate::stream::DiagnoseRepeat;
use std::time::Instant;

#[test]
fn no_diagnose_notice_is_due_without_a_run() {
    assert_eq!(DiagnoseRepeat::new(None).due(Instant::now()), None);
}
