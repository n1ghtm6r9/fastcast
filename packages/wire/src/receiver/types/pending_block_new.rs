use crate::packet::SHARD_SIZE;
use crate::receiver::PendingBlock;

impl PendingBlock {
    pub fn new(data_shards: usize, parity_shards: usize) -> PendingBlock {
        let total = data_shards + parity_shards;

        PendingBlock {
            data_shards,
            parity_shards,
            present: vec![false; total],
            present_count: 0,
            data_present: 0,
            bytes: vec![0; total * SHARD_SIZE],
        }
    }
}
