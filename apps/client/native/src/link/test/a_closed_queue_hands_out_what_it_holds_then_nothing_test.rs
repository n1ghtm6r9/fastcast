use crate::link::test::config_frame;
use crate::link::{EventQueue, LinkEvent};
use std::time::{Duration, Instant};

#[test]
fn a_closed_queue_hands_out_what_it_holds_then_nothing() {
    let queue = EventQueue::new();
    queue.push(LinkEvent::Connected);
    queue.push_frame(config_frame(b"{}"));
    queue.close();
    let started = Instant::now();

    let first = queue.next(Duration::from_secs(1));
    let second = queue.next(Duration::from_secs(1));
    let third = queue.next(Duration::from_secs(1));

    assert!(queue.is_closed());
    assert_eq!(first, Some(LinkEvent::Connected));
    assert_eq!(second, Some(LinkEvent::Config(b"{}".to_vec())));
    assert_eq!(third, None);
    assert!(started.elapsed() < Duration::from_millis(10));
}
