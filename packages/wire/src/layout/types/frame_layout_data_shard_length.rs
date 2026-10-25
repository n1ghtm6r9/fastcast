use crate::layout::FrameLayout;
use crate::packet::SHARD_SIZE;

impl FrameLayout {
    pub fn data_shard_length(&self, data_shard: usize) -> usize {
        (self.frame_size as usize)
            .saturating_sub(data_shard.saturating_mul(SHARD_SIZE))
            .min(SHARD_SIZE)
    }
}
