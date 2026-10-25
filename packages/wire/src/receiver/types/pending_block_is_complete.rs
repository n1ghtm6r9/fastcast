use crate::receiver::PendingBlock;

impl PendingBlock {
    pub fn is_complete(&self) -> bool {
        self.data_present == self.data_shards
    }
}
