use crate::link::test::{push_video_frames, video_frame};
use crate::link::{EventQueue, LinkEvent, MAX_QUEUED_FRAMES};
use std::time::Duration;

#[test]
fn a_notice_is_kept_when_the_queue_drops_its_frames() {
    let queue = EventQueue::new();
    queue.push(LinkEvent::Notice("{}".to_string()));
    push_video_frames(&queue, MAX_QUEUED_FRAMES);

    queue.push_frame(video_frame(false, &[99]));

    assert_eq!(
        queue.next(Duration::ZERO),
        Some(LinkEvent::Notice("{}".to_string()))
    );
    assert_eq!(queue.next(Duration::ZERO), None);
}
