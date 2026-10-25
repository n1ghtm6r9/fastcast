use crate::fec::MAX_DATA_SHARDS;
use crate::layout::{FrameLayout, MAX_FRAME_SIZE};
use crate::packet::SHARD_SIZE;

#[test]
fn blocks_partition_the_data_shards_in_order_and_differ_by_at_most_one() {
    let boundary_sizes = (0..=300usize)
        .flat_map(|shards| [shards * SHARD_SIZE, shards * SHARD_SIZE + 1])
        .chain([MAX_FRAME_SIZE - 1, MAX_FRAME_SIZE]);

    for frame_size in boundary_sizes {
        let layout = FrameLayout::new(frame_size as u32);
        let sizes: Vec<usize> = (0..layout.blocks)
            .map(|block| layout.data_shards_in(block))
            .collect();
        let mut next_first = 0;

        for (block, size) in sizes.iter().enumerate() {
            assert_eq!(layout.first_data_shard_of(block), next_first);
            assert!((1..=MAX_DATA_SHARDS).contains(size));
            next_first += size;
        }

        assert_eq!(next_first, layout.data_shards);
        assert!(sizes.iter().max().unwrap() - sizes.iter().min().unwrap() <= 1);
    }
}
