use crate::packet::ShardHeader;
use crate::receiver::PendingFrame;

impl PendingFrame {
    pub fn matches(&self, header: &ShardHeader) -> bool {
        self.kind == header.frame_kind
            && self.keyframe == header.keyframe
            && self.layout.frame_size == header.frame_size
    }
}
