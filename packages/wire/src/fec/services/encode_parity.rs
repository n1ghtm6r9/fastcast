use crate::fec::{block_length, cauchy_coefficient};
use crate::gf::mul_add_slice;

pub fn encode_parity(
    data_shards: usize,
    parity_shards: usize,
    shard_size: usize,
    block: &mut [u8],
) {
    let Some(length) = block_length(data_shards, parity_shards, shard_size) else {
        return;
    };
    let Some(block) = block.get_mut(..length) else {
        return;
    };

    if shard_size == 0 {
        return;
    }

    let (data, parity) = block.split_at_mut(data_shards * shard_size);

    parity.fill(0);

    for (column, data_shard) in data.chunks_exact(shard_size).enumerate() {
        for (row, parity_shard) in parity.chunks_exact_mut(shard_size).enumerate() {
            mul_add_slice(cauchy_coefficient(row, column), data_shard, parity_shard);
        }
    }
}
