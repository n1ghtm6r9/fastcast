use crate::control::MAX_FEC_PERCENT;
use crate::layout::{FrameLayout, MAX_FRAME_SIZE, parity_shards_for};
use crate::packet::SHARD_SIZE;
use crate::receiver::MAX_PENDING_BYTES;

#[test]
fn a_full_size_frame_at_the_maximum_fec_level_fits_the_byte_budget() {
    let layout = FrameLayout::new(MAX_FRAME_SIZE as u32);

    let bytes: usize = (0..layout.blocks)
        .map(|block| {
            let data_shards = layout.data_shards_in(block);

            (data_shards + parity_shards_for(data_shards, MAX_FEC_PERCENT)) * SHARD_SIZE
        })
        .sum();

    assert!(bytes <= MAX_PENDING_BYTES);
}
