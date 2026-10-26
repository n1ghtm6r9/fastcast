use crate::link::EventQueue;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn closing_wakes_a_blocked_reader() {
    let queue = Arc::new(EventQueue::new());
    let reader = {
        let queue = Arc::clone(&queue);
        thread::spawn(move || queue.next(Duration::from_secs(5)))
    };
    thread::sleep(Duration::from_millis(50));
    let closed_at = Instant::now();

    queue.close();

    assert_eq!(reader.join().unwrap(), None);
    assert!(closed_at.elapsed() < Duration::from_millis(500));
}
