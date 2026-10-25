use crate::layout::{FrameLayout, parity_shards_for};
use crate::packet::SHARD_SIZE;
use crate::packetizer::BlockPlan;

pub fn plan_blocks(layout: &FrameLayout, fec_percent: u8, plans: &mut Vec<BlockPlan>) -> usize {
    plans.clear();

    let mut scratch_shards = 0;

    for block in 0..layout.blocks {
        let data_shards = layout.data_shards_in(block);
        let parity_shards = parity_shards_for(data_shards, fec_percent);

        plans.push(BlockPlan {
            first_data_shard: layout.first_data_shard_of(block),
            data_shards,
            parity_shards,
            scratch_offset: scratch_shards * SHARD_SIZE,
        });

        if parity_shards > 0 {
            scratch_shards += data_shards + parity_shards;
        }
    }

    scratch_shards * SHARD_SIZE
}
