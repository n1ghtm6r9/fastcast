use crate::packet::{FrameKind, Shard, ShardHeader};
use crate::receiver::FrameReceiver;
use std::time::Instant;

#[test]
fn shards_with_impossible_headers_are_ignored() {
    let valid = ShardHeader {
        frame: 0,
        frame_kind: FrameKind::Video,
        keyframe: true,
        block: 0,
        blocks: 1,
        data_shards: 1,
        parity_shards: 1,
        shard: 0,
        frame_size: 100,
    };
    let data = [0u8; 100];
    let impossible = [
        Shard {
            header: ShardHeader { blocks: 2, ..valid },
            data: &data,
        },
        Shard {
            header: ShardHeader {
                data_shards: 2,
                ..valid
            },
            data: &data,
        },
        Shard {
            header: ShardHeader { shard: 5, ..valid },
            data: &data,
        },
        Shard {
            header: valid,
            data: &data[..99],
        },
        Shard {
            header: ShardHeader {
                parity_shards: 128,
                ..valid
            },
            data: &data,
        },
    ];

    for shard in &impossible {
        let mut receiver = FrameReceiver::new();

        receiver.accept(shard, Instant::now());

        assert_eq!(receiver.next_deadline(), None);
    }
}
