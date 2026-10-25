use crate::fec::MAX_PARITY_SHARDS;
use crate::layout::{FrameLayout, MAX_FRAME_SIZE};
use crate::packet::Shard;
use crate::receiver::shard_length;

pub fn shard_layout(shard: &Shard<'_>) -> Option<FrameLayout> {
    let header = &shard.header;

    if header.frame_size as usize > MAX_FRAME_SIZE {
        return None;
    }

    let layout = FrameLayout::new(header.frame_size);
    let block = usize::from(header.block);
    let data_shards = usize::from(header.data_shards);
    let parity_shards = usize::from(header.parity_shards);
    let consistent = usize::from(header.blocks) == layout.blocks
        && block < layout.blocks
        && data_shards == layout.data_shards_in(block)
        && parity_shards <= MAX_PARITY_SHARDS
        && usize::from(header.shard) < data_shards + parity_shards
        && shard.data.len() == shard_length(&layout, header);

    consistent.then_some(layout)
}
