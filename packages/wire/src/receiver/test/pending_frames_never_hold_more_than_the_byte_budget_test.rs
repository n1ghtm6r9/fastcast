use crate::control::MAX_FEC_PERCENT;
use crate::layout::{FrameLayout, MAX_FRAME_SIZE, parity_shards_for};
use crate::packet::{FrameKind, SHARD_SIZE, Shard, ShardHeader};
use crate::receiver::{FrameReceiver, MAX_PENDING_BYTES, PendingFrame};
use std::time::Instant;

#[test]
fn pending_frames_never_hold_more_than_the_byte_budget() {
    let layout = FrameLayout::new(MAX_FRAME_SIZE as u32);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for frame in 0..8 {
        for block in 0..layout.blocks {
            let data_shards = layout.data_shards_in(block);
            let shard = Shard {
                header: ShardHeader {
                    frame,
                    frame_kind: FrameKind::Video,
                    keyframe: true,
                    block: block as u16,
                    blocks: layout.blocks as u16,
                    data_shards: data_shards as u8,
                    parity_shards: parity_shards_for(data_shards, MAX_FEC_PERCENT) as u8,
                    shard: data_shards as u8,
                    frame_size: MAX_FRAME_SIZE as u32,
                },
                data: &[0; SHARD_SIZE],
            };

            receiver.accept(&shard, t0);

            let held: usize = receiver
                .pending
                .values()
                .map(PendingFrame::allocated_bytes)
                .sum();
            assert_eq!(receiver.pending_bytes, held);
            assert!(receiver.pending_bytes <= MAX_PENDING_BYTES);
        }
    }

    assert!(receiver.lost_frames() > 0);
}
