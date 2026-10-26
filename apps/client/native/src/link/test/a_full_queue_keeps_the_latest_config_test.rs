use crate::link::test::{config_frame, push_video_frames, video_frame};
use crate::link::{EventQueue, LinkEvent, MAX_QUEUED_FRAMES};
use std::time::Duration;

#[test]
fn a_full_queue_keeps_the_latest_config() {
    let queue = EventQueue::new();
    queue.push(LinkEvent::Connected);
    queue.push_frame(config_frame(&[1]));
    queue.push_frame(config_frame(&[2]));
    push_video_frames(&queue, MAX_QUEUED_FRAMES - 2);

    queue.push_frame(video_frame(false, &[99]));
    queue.push_frame(video_frame(true, &[101]));

    let events: Vec<LinkEvent> = std::iter::from_fn(|| queue.next(Duration::ZERO)).collect();
    assert_eq!(
        events,
        vec![
            LinkEvent::Connected,
            LinkEvent::Config(vec![2]),
            LinkEvent::Video(vec![101]),
        ]
    );
}
