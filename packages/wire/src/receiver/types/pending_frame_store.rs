use crate::packet::Shard;
use crate::receiver::{PendingBlock, PendingFrame};
use std::time::Instant;

impl PendingFrame {
    pub fn store(&mut self, shard: &Shard<'_>, now: Instant) -> usize {
        let header = &shard.header;
        let block = self.blocks[usize::from(header.block)].get_or_insert_with(|| {
            PendingBlock::new(
                usize::from(header.data_shards),
                usize::from(header.parity_shards),
            )
        });

        if block.parity_shards != usize::from(header.parity_shards) {
            return 0;
        }

        let was_complete = block.is_complete();
        let rebuilt = block.store(usize::from(header.shard), shard.data);

        if !was_complete && block.is_complete() {
            self.complete_blocks += 1;

            if self.complete_blocks == self.blocks.len() {
                self.completed_at = Some(now);
            }
        }

        rebuilt
    }
}
