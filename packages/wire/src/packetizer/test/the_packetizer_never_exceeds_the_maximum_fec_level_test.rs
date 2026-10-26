use crate::control::MAX_FEC_PERCENT;
use crate::layout::parity_shards_for;
use crate::packet::SHARD_SIZE;
use crate::packetizer::test::{shard_of, video_frame};
use crate::packetizer::{Datagrams, Packetizer};
use std::collections::BTreeMap;

#[test]
fn the_packetizer_never_exceeds_the_maximum_fec_level() {
    let payload = vec![1u8; 129 * SHARD_SIZE];
    let mut packetizer = Packetizer::new(1);
    let mut out = Datagrams::new();
    let mut blocks: BTreeMap<u16, (usize, usize)> = BTreeMap::new();

    packetizer.frame(&video_frame(&payload, true), 90, &mut out);

    for shard in out.iter().map(shard_of) {
        let header = shard.header;
        let counts = blocks
            .entry(header.block)
            .or_insert((usize::from(header.data_shards), 0));

        counts.1 += usize::from(header.shard >= header.data_shards);
    }

    assert_eq!(blocks.len(), 2);
    for (data_shards, parity_shards) in blocks.into_values() {
        assert_eq!(
            parity_shards,
            parity_shards_for(data_shards, MAX_FEC_PERCENT)
        );
    }
}
