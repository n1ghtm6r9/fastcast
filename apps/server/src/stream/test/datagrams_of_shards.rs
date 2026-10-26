use wire::{Datagrams, FrameKind, OutgoingFrame, Packetizer, SHARD_SIZE};

pub fn datagrams_of_shards(count: usize) -> Datagrams {
    let payload = vec![7u8; count * SHARD_SIZE];
    let mut datagrams = Datagrams::new();

    Packetizer::new(1).frame(
        &OutgoingFrame {
            kind: FrameKind::Video,
            keyframe: false,
            payload: &payload,
        },
        0,
        &mut datagrams,
    );

    datagrams
}
