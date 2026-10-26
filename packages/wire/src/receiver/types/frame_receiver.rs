use crate::receiver::PendingFrame;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct FrameReceiver {
    pub(crate) expected: u64,
    pub(crate) highest: u32,
    pub(crate) pending: BTreeMap<u64, PendingFrame>,
    pub(crate) waiting_for_keyframe: bool,
    pub(crate) video_lost_while_waiting: bool,
    pub(crate) config_lost: bool,
    pub(crate) lost_frames: u32,
    pub(crate) recovered_shards: u32,
}
