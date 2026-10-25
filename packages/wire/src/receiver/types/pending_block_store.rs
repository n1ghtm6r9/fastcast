use crate::fec::recover_data;
use crate::packet::SHARD_SIZE;
use crate::receiver::PendingBlock;

impl PendingBlock {
    pub fn store(&mut self, shard: usize, data: &[u8]) -> usize {
        if self.is_complete() || self.present[shard] {
            return 0;
        }

        let start = shard * SHARD_SIZE;

        self.bytes[start..start + data.len()].copy_from_slice(data);
        self.present[shard] = true;
        self.present_count += 1;
        self.data_present += usize::from(shard < self.data_shards);

        if self.is_complete() || self.present_count < self.data_shards {
            return 0;
        }

        let Some(rebuilt) = recover_data(
            self.data_shards,
            self.parity_shards,
            SHARD_SIZE,
            &mut self.bytes,
            &self.present,
        ) else {
            return 0;
        };

        self.present[..self.data_shards].fill(true);
        self.data_present = self.data_shards;

        rebuilt
    }
}
