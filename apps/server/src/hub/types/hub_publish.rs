use crate::hub::Hub;

impl Hub {
    pub fn publish(&self, key: bool, timestamp_ms: u32, data: &[u8]) {
        self.publish_shown(key, timestamp_ms, data, None);
    }
}
