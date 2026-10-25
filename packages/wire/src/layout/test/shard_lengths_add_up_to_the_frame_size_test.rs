use crate::layout::{FrameLayout, MAX_FRAME_SIZE};
use crate::packet::SHARD_SIZE;

#[test]
fn shard_lengths_add_up_to_the_frame_size() {
    let boundary_sizes = (0..=300usize)
        .flat_map(|shards| [shards * SHARD_SIZE, shards * SHARD_SIZE + 1])
        .chain([MAX_FRAME_SIZE - 1, MAX_FRAME_SIZE]);

    for frame_size in boundary_sizes {
        let layout = FrameLayout::new(frame_size as u32);
        let total: usize = (0..layout.data_shards)
            .map(|shard| layout.data_shard_length(shard))
            .sum();

        assert_eq!(total, frame_size);
    }
}
