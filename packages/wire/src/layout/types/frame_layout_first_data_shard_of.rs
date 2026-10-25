use crate::layout::FrameLayout;

impl FrameLayout {
    pub fn first_data_shard_of(&self, block: usize) -> usize {
        let base = self.data_shards / self.blocks;
        let larger_blocks = self.data_shards % self.blocks;

        block * base + block.min(larger_blocks)
    }
}
