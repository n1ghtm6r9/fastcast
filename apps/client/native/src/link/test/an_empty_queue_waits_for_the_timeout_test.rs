use crate::link::EventQueue;
use std::time::{Duration, Instant};

#[test]
fn an_empty_queue_waits_for_the_timeout() {
    let queue = EventQueue::new();
    let started = Instant::now();

    let event = queue.next(Duration::from_millis(30));

    assert_eq!(event, None);
    assert!(started.elapsed() >= Duration::from_millis(30));
}
