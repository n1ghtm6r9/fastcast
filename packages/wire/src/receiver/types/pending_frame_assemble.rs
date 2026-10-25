use crate::packet::SHARD_SIZE;
use crate::receiver::PendingFrame;

impl PendingFrame {
    pub fn assemble(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(self.layout.frame_size as usize);

        for (index, block) in self.blocks.iter().enumerate() {
            let Some(block) = block else {
                continue;
            };
            let first = self.layout.first_data_shard_of(index);

            for shard in 0..block.data_shards {
                let start = shard * SHARD_SIZE;
                let length = self.layout.data_shard_length(first + shard);

                payload.extend_from_slice(&block.bytes[start..start + length]);
            }
        }

        payload
    }
}
