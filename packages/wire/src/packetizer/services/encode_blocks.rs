use crate::fec::encode_parity;
use crate::layout::FrameLayout;
use crate::packet::SHARD_SIZE;
use crate::packetizer::BlockPlan;

pub fn encode_blocks(
    payload: &[u8],
    layout: &FrameLayout,
    plans: &[BlockPlan],
    scratch: &mut [u8],
) {
    for plan in plans.iter().filter(|plan| plan.parity_shards > 0) {
        let block = &mut scratch
            [plan.scratch_offset..plan.scratch_offset + plan.total_shards() * SHARD_SIZE];

        for shard in 0..plan.data_shards {
            let data_shard = plan.first_data_shard + shard;
            let start = data_shard * SHARD_SIZE;
            let length = layout.data_shard_length(data_shard);

            block[shard * SHARD_SIZE..shard * SHARD_SIZE + length]
                .copy_from_slice(&payload[start..start + length]);
        }

        encode_parity(plan.data_shards, plan.parity_shards, SHARD_SIZE, block);
    }
}
