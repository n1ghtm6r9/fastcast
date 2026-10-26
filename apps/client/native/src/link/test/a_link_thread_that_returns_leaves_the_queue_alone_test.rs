use crate::link::{LinkShared, end_link_on_panic};
use std::time::Duration;

#[test]
fn a_link_thread_that_returns_leaves_the_queue_alone() {
    let shared = LinkShared::default();

    end_link_on_panic(&shared, || {});

    assert_eq!(shared.queue.next(Duration::ZERO), None);
    assert!(!shared.queue.is_closed());
}
