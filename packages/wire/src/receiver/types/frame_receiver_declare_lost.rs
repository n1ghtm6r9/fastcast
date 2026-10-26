use crate::packet::FrameKind;
use crate::receiver::FrameReceiver;
use std::mem::replace;

impl FrameReceiver {
    pub(crate) fn declare_lost(&mut self, end: u64) {
        let count = end - self.expected;
        let kept = self.pending.split_off(&end);
        let dropped = replace(&mut self.pending, kept);
        let never_seen = (dropped.len() as u64) < count;
        let any_video = dropped.values().any(|frame| frame.kind == FrameKind::Video);
        let any_config = dropped
            .values()
            .any(|frame| frame.kind == FrameKind::Config);

        self.lost_frames = self
            .lost_frames
            .saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
        self.waiting_for_keyframe = true;
        self.video_lost_while_waiting |= never_seen || any_video;
        self.config_lost |= never_seen || any_config;
        self.expected = end;
    }
}
