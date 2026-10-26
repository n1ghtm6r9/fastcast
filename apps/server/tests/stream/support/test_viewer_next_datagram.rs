use crate::support::{TestViewer, receive_within};
use std::time::Duration;

impl TestViewer {
    pub fn next_datagram(&self, timeout: Duration) -> Option<Vec<u8>> {
        receive_within(&self.socket, timeout).map(|(bytes, _)| bytes)
    }
}
