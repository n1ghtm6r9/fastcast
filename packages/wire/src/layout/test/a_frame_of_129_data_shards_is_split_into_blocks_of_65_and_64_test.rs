use crate::layout::FrameLayout;
use crate::packet::SHARD_SIZE;

#[test]
fn a_frame_of_129_data_shards_is_split_into_blocks_of_65_and_64() {
    let layout = FrameLayout::new(129 * SHARD_SIZE as u32);

    assert_eq!(layout.data_shards, 129);
    assert_eq!(layout.blocks, 2);
    assert_eq!(layout.data_shards_in(0), 65);
    assert_eq!(layout.data_shards_in(1), 64);
    assert_eq!(layout.first_data_shard_of(0), 0);
    assert_eq!(layout.first_data_shard_of(1), 65);
}
