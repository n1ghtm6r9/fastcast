use crate::receiver::{FrameReceiver, PendingFrame};
use std::collections::btree_map::Entry;

impl FrameReceiver {
    pub(crate) fn take_complete_front(&mut self) -> Option<PendingFrame> {
        let Entry::Occupied(entry) = self.pending.entry(self.expected) else {
            return None;
        };

        if !entry.get().is_complete() {
            return None;
        }

        self.expected += 1;

        let frame = entry.remove();

        self.pending_bytes = self.pending_bytes.saturating_sub(frame.allocated_bytes());

        Some(frame)
    }
}
