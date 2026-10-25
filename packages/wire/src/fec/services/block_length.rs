use crate::fec::{MAX_DATA_SHARDS, MAX_PARITY_SHARDS};

pub fn block_length(data_shards: usize, parity_shards: usize, shard_size: usize) -> Option<usize> {
    if data_shards > MAX_DATA_SHARDS || parity_shards > MAX_PARITY_SHARDS {
        return None;
    }

    (data_shards + parity_shards).checked_mul(shard_size)
}
