use crate::capture::StopOnDrop;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
fn dropping_the_guard_sets_the_stop_flag() {
    let flag = Arc::new(AtomicBool::new(false));

    drop(StopOnDrop(flag.clone()));

    assert!(flag.load(Ordering::Acquire));
}
