use crate::layout::FrameLayout;

#[test]
fn an_empty_frame_still_has_one_data_shard() {
    let layout = FrameLayout::new(0);

    assert_eq!(layout.data_shards, 1);
    assert_eq!(layout.blocks, 1);
    assert_eq!(layout.data_shard_length(0), 0);
}
