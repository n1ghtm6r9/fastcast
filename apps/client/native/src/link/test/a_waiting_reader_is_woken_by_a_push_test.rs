use crate::link::test::video_frame;
use crate::link::{EventQueue, LinkEvent};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn a_waiting_reader_is_woken_by_a_push() {
    let queue = Arc::new(EventQueue::new());
    let reader = {
        let queue = Arc::clone(&queue);
        thread::spawn(move || {
            let started = Instant::now();
            let event = queue.next(Duration::from_secs(2));
            (event, started.elapsed())
        })
    };
    thread::sleep(Duration::from_millis(50));

    queue.push_frame(video_frame(true, &[7]));

    let (event, waited) = reader.join().unwrap();
    assert_eq!(event, Some(LinkEvent::Video(vec![7])));
    assert!(waited < Duration::from_secs(1));
}
