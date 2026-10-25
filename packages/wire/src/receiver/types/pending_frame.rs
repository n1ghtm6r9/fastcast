use crate::layout::FrameLayout;
use crate::packet::FrameKind;
use crate::receiver::PendingBlock;
use std::time::Instant;

#[derive(Debug)]
pub struct PendingFrame {
    pub(crate) number: u32,
    pub(crate) kind: FrameKind,
    pub(crate) keyframe: bool,
    pub(crate) layout: FrameLayout,
    pub(crate) blocks: Vec<Option<PendingBlock>>,
    pub(crate) complete_blocks: usize,
    pub(crate) first_seen: Instant,
    pub(crate) completed_at: Option<Instant>,
}
