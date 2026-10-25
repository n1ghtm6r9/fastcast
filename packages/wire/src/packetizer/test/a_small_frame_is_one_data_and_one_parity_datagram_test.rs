use crate::packet::{FrameKind, SHARD_SIZE, ShardHeader};
use crate::packetizer::test::{shard_of, video_frame};
use crate::packetizer::{Datagrams, Packetizer};

#[test]
fn a_small_frame_is_one_data_and_one_parity_datagram() {
    let payload: Vec<u8> = (0..100).collect();
    let mut packetizer = Packetizer::new(7);
    let mut out = Datagrams::new();

    packetizer.frame(&video_frame(&payload, true), 10, &mut out);

    assert_eq!(out.len(), 2);
    let first = shard_of(out.get(0));
    assert_eq!(
        first.header,
        ShardHeader {
            frame: 0,
            frame_kind: FrameKind::Video,
            keyframe: true,
            block: 0,
            blocks: 1,
            data_shards: 1,
            parity_shards: 1,
            shard: 0,
            frame_size: 100
        }
    );
    assert_eq!(first.data, payload);
    let second = shard_of(out.get(1));
    assert_eq!((second.header.shard, second.data.len()), (1, SHARD_SIZE));
}
