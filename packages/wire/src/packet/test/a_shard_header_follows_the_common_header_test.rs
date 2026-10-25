use crate::packet::{Datagram, FrameKind, Packet, Shard, ShardHeader, encode_datagram};

#[test]
fn a_shard_header_follows_the_common_header() {
    let header = ShardHeader {
        frame: 7,
        frame_kind: FrameKind::Config,
        keyframe: true,
        block: 1,
        blocks: 2,
        data_shards: 3,
        parity_shards: 1,
        shard: 2,
        frame_size: 5000,
    };
    let mut out = Vec::new();

    encode_datagram(
        &Datagram {
            session: 9,
            sequence: 1,
            packet: Packet::Shard(Shard {
                header,
                data: &[0xAA],
            }),
        },
        &mut out,
    );

    assert_eq!(out[..3], [0xFC, 1, 3]);
    assert_eq!(
        out[12..],
        [
            7, 0, 0, 0, 1, 1, 1, 0, 2, 0, 3, 1, 2, 0, 0x88, 0x13, 0, 0, 0xAA
        ]
    );
}
