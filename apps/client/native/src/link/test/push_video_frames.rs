use crate::link::EventQueue;
use crate::link::test::video_frame;

pub(crate) fn push_video_frames(queue: &EventQueue, count: usize) {
    for index in 0..count {
        queue.push_frame(video_frame(index == 0, &[index as u8]));
    }
}
