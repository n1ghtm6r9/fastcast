use crate::packet::{FrameKind, ShardHeader};
use crate::packetizer::Packetizer;
use crate::packetizer::test::shard_of;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed};
use std::time::Instant;

#[test]
fn shards_that_disagree_with_their_frame_are_ignored() {
    let mutations: [fn(&mut ShardHeader); 3] = [
        |header| header.keyframe = !header.keyframe,
        |header| header.frame_kind = FrameKind::Config,
        |header| header.frame_size -= 1,
    ];

    for mutate in mutations {
        let mut packetizer = Packetizer::new(1);
        let mut receiver = FrameReceiver::new();
        let t0 = Instant::now();
        let datagrams = datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 3000], 0);
        let mut disagreeing = shard_of(&datagrams[1]);
        mutate(&mut disagreeing.header);

        feed(&mut receiver, &datagrams[0], t0);
        feed(&mut receiver, &datagrams[2], t0);
        receiver.accept(&disagreeing, t0);

        assert!(deliveries(&mut receiver, t0).is_empty());

        feed(&mut receiver, &datagrams[1], t0);

        assert_eq!(deliveries(&mut receiver, t0).len(), 1);
    }
}
