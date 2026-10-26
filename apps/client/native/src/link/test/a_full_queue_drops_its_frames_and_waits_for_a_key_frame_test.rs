use crate::link::test::{push_video_frames, video_frame};
use crate::link::{EventQueue, LinkEvent, MAX_QUEUED_FRAMES};
use std::time::Duration;

#[test]
fn a_full_queue_drops_its_frames_and_waits_for_a_key_frame() {
    let queue = EventQueue::new();
    queue.push(LinkEvent::Connected);
    push_video_frames(&queue, MAX_QUEUED_FRAMES);
    assert!(!queue.needs_key_frame());

    queue.push_frame(video_frame(false, &[99]));

    assert!(queue.needs_key_frame());
    assert_eq!(queue.next(Duration::ZERO), Some(LinkEvent::Connected));
    assert_eq!(queue.next(Duration::ZERO), None);

    queue.push_frame(video_frame(false, &[100]));

    assert_eq!(queue.next(Duration::ZERO), None);
    assert!(queue.needs_key_frame());

    queue.push_frame(video_frame(true, &[101]));

    assert!(!queue.needs_key_frame());
    assert_eq!(
        queue.next(Duration::ZERO),
        Some(LinkEvent::Video(vec![101]))
    );
}
