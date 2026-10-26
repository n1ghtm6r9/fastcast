use crate::link::test::{TEST_WAIT, config_frame};
use crate::link::{EventQueue, LinkEvent};
use std::time::Instant;

#[test]
fn a_closed_queue_hands_out_what_it_holds_then_nothing() {
    let queue = EventQueue::new();
    queue.push(LinkEvent::Connected);
    queue.push_frame(config_frame(b"{}"));
    queue.close();
    let patience = TEST_WAIT * 2;
    let started = Instant::now();

    let first = queue.next(patience);
    let second = queue.next(patience);
    let third = queue.next(patience);

    assert!(queue.is_closed());
    assert_eq!(first, Some(LinkEvent::Connected));
    assert_eq!(second, Some(LinkEvent::Config(b"{}".to_vec())));
    assert_eq!(third, None);
    assert!(started.elapsed() < TEST_WAIT);
}
