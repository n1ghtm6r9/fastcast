use crate::packet::Shard;
use crate::receiver::{FrameReceiver, PendingFrame, shard_layout};
use std::collections::btree_map::Entry;
use std::time::Instant;

impl FrameReceiver {
    pub fn accept(&mut self, shard: &Shard<'_>, now: Instant) {
        let header = &shard.header;
        let number = u64::from(header.frame);

        if number < self.expected {
            return;
        }

        let Some(layout) = shard_layout(shard) else {
            return;
        };

        let frame = match self.pending.entry(number) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(PendingFrame::new(header, layout, now)),
        };

        if frame.is_complete() || !frame.matches(header) {
            return;
        }

        let rebuilt = frame.store(shard, now);

        self.recovered_shards = self.recovered_shards.saturating_add(rebuilt as u32);
        self.enforce_pending_limit();
    }
}
