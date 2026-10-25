use crate::fec::MAX_DATA_SHARDS;
use crate::layout::FrameLayout;
use crate::packet::SHARD_SIZE;

impl FrameLayout {
    pub fn new(frame_size: u32) -> FrameLayout {
        let data_shards = (frame_size as usize).div_ceil(SHARD_SIZE).max(1);

        FrameLayout {
            frame_size,
            data_shards,
            blocks: data_shards.div_ceil(MAX_DATA_SHARDS),
        }
    }
}
