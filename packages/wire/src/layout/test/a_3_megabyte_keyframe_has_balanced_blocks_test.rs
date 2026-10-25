use crate::layout::FrameLayout;

#[test]
fn a_3_megabyte_keyframe_has_balanced_blocks() {
    let layout = FrameLayout::new(3_000_000);

    assert_eq!(layout.data_shards, 2190);
    assert_eq!(layout.blocks, 18);
    assert!((0..12).all(|block| layout.data_shards_in(block) == 122));
    assert!((12..18).all(|block| layout.data_shards_in(block) == 121));
}
