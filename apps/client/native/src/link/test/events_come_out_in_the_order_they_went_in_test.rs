use crate::link::test::{config_frame, video_frame};
use crate::link::{EventQueue, LinkEvent};
use std::time::Duration;

#[test]
fn events_come_out_in_the_order_they_went_in() {
    let queue = EventQueue::new();

    queue.push(LinkEvent::Connected);
    queue.push_frame(config_frame(b"{}"));
    queue.push_frame(video_frame(true, &[1]));
    queue.push_frame(video_frame(false, &[2]));

    let events: Vec<Option<LinkEvent>> = (0..5).map(|_| queue.next(Duration::ZERO)).collect();

    assert_eq!(
        events,
        vec![
            Some(LinkEvent::Connected),
            Some(LinkEvent::Config(b"{}".to_vec())),
            Some(LinkEvent::Video(vec![1])),
            Some(LinkEvent::Video(vec![2])),
            None,
        ]
    );
}
