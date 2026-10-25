use crate::packetizer::test::{shard_of, video_frame};
use crate::packetizer::{Datagrams, Packetizer};
use crate::test_support::random_bytes;

#[test]
fn data_shards_carry_the_payload_in_order() {
    let mut state = 5;
    let payload = random_bytes(&mut state, 600_000);
    let mut packetizer = Packetizer::new(1);
    let mut out = Datagrams::new();

    packetizer.frame(&video_frame(&payload, false), 10, &mut out);

    let mut data_shards: Vec<(u16, u8, Vec<u8>)> = out
        .iter()
        .map(shard_of)
        .filter(|shard| shard.header.shard < shard.header.data_shards)
        .map(|shard| (shard.header.block, shard.header.shard, shard.data.to_vec()))
        .collect();
    data_shards.sort_by_key(|(block, shard, _)| (*block, *shard));
    let rebuilt: Vec<u8> = data_shards
        .into_iter()
        .flat_map(|(_, _, data)| data)
        .collect();

    assert_eq!(rebuilt, payload);
}
