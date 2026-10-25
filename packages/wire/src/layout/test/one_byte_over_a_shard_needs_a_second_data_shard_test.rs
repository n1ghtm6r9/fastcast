use crate::layout::FrameLayout;
use crate::packet::SHARD_SIZE;

#[test]
fn one_byte_over_a_shard_needs_a_second_data_shard() {
    let layout = FrameLayout::new(SHARD_SIZE as u32 + 1);

    assert_eq!(layout.data_shards, 2);
    assert_eq!(layout.data_shard_length(0), SHARD_SIZE);
    assert_eq!(layout.data_shard_length(1), 1);
}
