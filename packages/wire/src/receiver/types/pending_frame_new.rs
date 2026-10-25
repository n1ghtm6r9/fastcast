use crate::layout::FrameLayout;
use crate::packet::ShardHeader;
use crate::receiver::PendingFrame;
use std::iter::repeat_with;
use std::time::Instant;

impl PendingFrame {
    pub fn new(header: &ShardHeader, layout: FrameLayout, now: Instant) -> PendingFrame {
        PendingFrame {
            number: header.frame,
            kind: header.frame_kind,
            keyframe: header.keyframe,
            layout,
            blocks: repeat_with(|| None).take(layout.blocks).collect(),
            complete_blocks: 0,
            first_seen: now,
            completed_at: None,
        }
    }
}
