use crate::receiver::FrameReceiver;
use std::collections::BTreeMap;

impl FrameReceiver {
    pub fn new() -> FrameReceiver {
        FrameReceiver {
            expected: 0,
            pending: BTreeMap::new(),
            waiting_for_keyframe: true,
            video_lost_while_waiting: false,
            config_lost: false,
            lost_frames: 0,
            recovered_shards: 0,
        }
    }
}
