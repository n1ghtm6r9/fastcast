use crate::layout::FrameLayout;
use crate::packet::SHARD_SIZE;
use crate::packetizer::BlockPlan;

pub fn shard_data<'a>(
    payload: &'a [u8],
    layout: &FrameLayout,
    plan: &BlockPlan,
    index: usize,
    scratch: &'a [u8],
) -> &'a [u8] {
    if index < plan.data_shards {
        let data_shard = plan.first_data_shard + index;
        let start = data_shard * SHARD_SIZE;

        &payload[start..start + layout.data_shard_length(data_shard)]
    } else {
        let start = plan.scratch_offset + index * SHARD_SIZE;

        &scratch[start..start + SHARD_SIZE]
    }
}
