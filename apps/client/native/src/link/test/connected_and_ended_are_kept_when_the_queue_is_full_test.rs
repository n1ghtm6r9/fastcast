use crate::link::test::push_video_frames;
use crate::link::{EventQueue, LinkEvent, MAX_QUEUED_FRAMES};
use std::time::Duration;

#[test]
fn connected_and_ended_are_kept_when_the_queue_is_full() {
    let queue = EventQueue::new();
    queue.push(LinkEvent::Connected);
    push_video_frames(&queue, MAX_QUEUED_FRAMES);

    queue.push(LinkEvent::Ended("x".to_string()));

    let events: Vec<LinkEvent> = std::iter::from_fn(|| queue.next(Duration::ZERO)).collect();
    assert_eq!(events.len(), MAX_QUEUED_FRAMES + 2);
    assert_eq!(events.first(), Some(&LinkEvent::Connected));
    assert_eq!(events.last(), Some(&LinkEvent::Ended("x".to_string())));
}
