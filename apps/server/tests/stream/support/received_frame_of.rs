use crate::support::{VideoFrame, video_frame_of};
use wire::ReceivedFrame;

pub fn received_frame_of(frame: &ReceivedFrame) -> VideoFrame {
    video_frame_of(&frame.payload)
}
