use super::frame_timestamp;
use crate::hub::Frame;
use std::sync::Arc;

pub fn timestamps_of(frames: &[Arc<Frame>]) -> Vec<u32> {
    frames.iter().map(|frame| frame_timestamp(frame)).collect()
}
