use crate::packet::{FrameKind, SHARD_SIZE, Shard, ShardHeader};
use crate::receiver::FrameReceiver;
use std::time::Instant;

#[test]
fn parity_beyond_the_maximum_fec_level_is_ignored() {
    let mut receiver = FrameReceiver::new();
    let shard = Shard {
        header: ShardHeader {
            frame: 0,
            frame_kind: FrameKind::Video,
            keyframe: true,
            block: 0,
            blocks: 1,
            data_shards: 128,
            parity_shards: 65,
            shard: 0,
            frame_size: 128 * SHARD_SIZE as u32,
        },
        data: &[0; SHARD_SIZE],
    };

    receiver.accept(&shard, Instant::now());

    assert_eq!(receiver.next_deadline(), None);
}
