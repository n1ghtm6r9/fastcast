use crate::layout::FrameLayout;

#[test]
fn a_data_shard_past_the_end_has_no_bytes() {
    let layout = FrameLayout::new(100);

    assert_eq!(layout.data_shard_length(1), 0);
    assert_eq!(layout.data_shard_length(usize::MAX), 0);
}
