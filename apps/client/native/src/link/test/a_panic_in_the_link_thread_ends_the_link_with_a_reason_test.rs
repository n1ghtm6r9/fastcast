use crate::link::{LinkEvent, LinkShared, RECEIVER_FAILED, end_link_on_panic};
use std::time::Duration;

#[test]
fn a_panic_in_the_link_thread_ends_the_link_with_a_reason() {
    let shared = LinkShared::default();

    end_link_on_panic(&shared, || panic!("the link thread broke"));

    assert_eq!(
        shared.queue.next(Duration::ZERO),
        Some(LinkEvent::Ended(RECEIVER_FAILED.to_string()))
    );
    assert!(shared.queue.is_closed());
}
