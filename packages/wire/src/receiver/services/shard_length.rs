use crate::layout::FrameLayout;
use crate::packet::{SHARD_SIZE, ShardHeader};

pub fn shard_length(layout: &FrameLayout, header: &ShardHeader) -> usize {
    let index = usize::from(header.shard);

    if index < usize::from(header.data_shards) {
        let first = layout.first_data_shard_of(usize::from(header.block));

        layout.data_shard_length(first + index)
    } else {
        SHARD_SIZE
    }
}
