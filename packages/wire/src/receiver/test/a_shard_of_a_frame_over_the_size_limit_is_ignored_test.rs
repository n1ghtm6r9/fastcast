use crate::layout::MAX_FRAME_SIZE;
use crate::packet::{FrameKind, Shard, ShardHeader};
use crate::receiver::FrameReceiver;
use std::time::Instant;

#[test]
fn a_shard_of_a_frame_over_the_size_limit_is_ignored() {
    let mut receiver = FrameReceiver::new();
    let shard = Shard {
        header: ShardHeader {
            frame: 0,
            frame_kind: FrameKind::Video,
            keyframe: true,
            block: 0,
            blocks: 1,
            data_shards: 1,
            parity_shards: 0,
            shard: 0,
            frame_size: MAX_FRAME_SIZE as u32 + 1,
        },
        data: &[0; 100],
    };

    receiver.accept(&shard, Instant::now());

    assert_eq!(receiver.next_deadline(), None);
    assert_eq!(receiver.highest_frame(), 0);
}
