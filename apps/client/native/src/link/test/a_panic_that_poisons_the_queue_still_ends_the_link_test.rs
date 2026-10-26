use crate::link::{LinkEvent, LinkShared, RECEIVER_FAILED, end_link_on_panic};
use std::time::Duration;

#[test]
fn a_panic_that_poisons_the_queue_still_ends_the_link() {
    let shared = LinkShared::default();

    end_link_on_panic(&shared, || {
        let _queue = shared.queue.lock();

        panic!("the link thread broke holding the queue");
    });

    assert_eq!(
        shared.queue.next(Duration::ZERO),
        Some(LinkEvent::Ended(RECEIVER_FAILED.to_string()))
    );
    assert!(shared.queue.is_closed());
}
