use crate::packet::{FrameKind, Shard, ShardHeader};
use crate::receiver::FrameReceiver;
use crate::receiver::test::{deliveries, ms};
use std::time::Instant;

#[test]
fn a_shard_numbered_four_billion_moves_the_expected_frame_in_one_step() {
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();
    let shard = Shard {
        header: ShardHeader {
            frame: 4_000_000_000,
            frame_kind: FrameKind::Video,
            keyframe: true,
            block: 0,
            blocks: 1,
            data_shards: 1,
            parity_shards: 0,
            shard: 0,
            frame_size: 100,
        },
        data: &[9; 100],
    };

    receiver.accept(&shard, t0);

    assert_eq!(receiver.lost_frames(), 3_999_999_937);
    assert_eq!(receiver.highest_frame(), 4_000_000_000);

    let numbers: Vec<u32> = deliveries(&mut receiver, t0 + ms(5))
        .iter()
        .map(|frame| frame.number)
        .collect();
    assert_eq!(numbers, [4_000_000_000]);
    assert_eq!(receiver.lost_frames(), 4_000_000_000);
}
