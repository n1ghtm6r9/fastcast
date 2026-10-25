use crate::hub::{Frame, Hub, encode_wire_frame, record_in_gop};
use std::sync::Arc;

impl Hub {
    pub fn publish(&self, key: bool, timestamp_ms: u32, data: &[u8]) {
        let wire = encode_wire_frame(key, timestamp_ms, data);
        let mut state = self.lock_state();

        let Some(config) = state.config.clone() else {
            return;
        };

        let frame = Arc::new(Frame { key, wire, config });

        record_in_gop(&mut state.gop, &frame);

        let _ = self.frames.send(frame);
    }
}
