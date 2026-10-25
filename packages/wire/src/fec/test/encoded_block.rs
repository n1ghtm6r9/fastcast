use crate::fec::encode_parity;
use crate::test_support::random_bytes;

pub fn encoded_block(
    data_shards: usize,
    parity_shards: usize,
    shard_size: usize,
    seed: u64,
) -> Vec<u8> {
    let mut state = seed;
    let mut block = random_bytes(&mut state, (data_shards + parity_shards) * shard_size);

    encode_parity(data_shards, parity_shards, shard_size, &mut block);

    block
}
