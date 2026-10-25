#[derive(Clone, Copy, Debug)]
pub struct BlockPlan {
    pub first_data_shard: usize,
    pub data_shards: usize,
    pub parity_shards: usize,
    pub scratch_offset: usize,
}
