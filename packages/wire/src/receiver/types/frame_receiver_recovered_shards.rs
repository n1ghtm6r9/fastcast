use crate::receiver::FrameReceiver;

impl FrameReceiver {
    pub fn recovered_shards(&self) -> u32 {
        self.recovered_shards
    }
}
