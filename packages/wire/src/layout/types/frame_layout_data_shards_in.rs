use crate::layout::FrameLayout;

impl FrameLayout {
    pub fn data_shards_in(&self, block: usize) -> usize {
        let base = self.data_shards / self.blocks;
        let larger_blocks = self.data_shards % self.blocks;

        base + usize::from(block < larger_blocks)
    }
}
