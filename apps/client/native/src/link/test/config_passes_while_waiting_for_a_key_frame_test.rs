use crate::link::test::{config_frame, push_video_frames, video_frame};
use crate::link::{EventQueue, LinkEvent, MAX_QUEUED_FRAMES};
use std::time::Duration;

#[test]
fn config_passes_while_waiting_for_a_key_frame() {
    let queue = EventQueue::new();
    push_video_frames(&queue, MAX_QUEUED_FRAMES);
    queue.push_frame(video_frame(false, &[99]));
    assert!(queue.needs_key_frame());

    queue.push_frame(config_frame(b"{}"));

    assert_eq!(
        queue.next(Duration::ZERO),
        Some(LinkEvent::Config(b"{}".to_vec()))
    );
    assert_eq!(queue.next(Duration::ZERO), None);
    assert!(queue.needs_key_frame());
}
