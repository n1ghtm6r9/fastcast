use crate::stream::DiagnoseRepeat;
use std::time::Instant;

#[test]
fn a_diagnose_notice_is_due_at_once() {
    assert_eq!(DiagnoseRepeat::new(Some(7)).due(Instant::now()), Some(7));
}
