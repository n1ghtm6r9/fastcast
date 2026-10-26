use crate::receiver::PendingFrame;

impl PendingFrame {
    pub fn allocated_bytes(&self) -> usize {
        self.blocks
            .iter()
            .flatten()
            .map(|block| block.bytes.len())
            .sum()
    }
}
