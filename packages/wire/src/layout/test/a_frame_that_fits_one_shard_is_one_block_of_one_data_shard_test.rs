use crate::layout::FrameLayout;

#[test]
fn a_frame_that_fits_one_shard_is_one_block_of_one_data_shard() {
    let layout = FrameLayout::new(100);

    assert_eq!(layout.data_shards, 1);
    assert_eq!(layout.blocks, 1);
    assert_eq!(layout.data_shards_in(0), 1);
    assert_eq!(layout.data_shard_length(0), 100);
}
