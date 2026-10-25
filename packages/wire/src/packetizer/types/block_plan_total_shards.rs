use crate::packetizer::BlockPlan;

impl BlockPlan {
    pub fn total_shards(&self) -> usize {
        self.data_shards + self.parity_shards
    }
}
