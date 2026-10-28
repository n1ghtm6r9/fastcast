use crate::hub::{Frame, Hub, encode_wire_frame, record_in_gop};
use std::sync::Arc;
use std::time::Instant;

impl Hub {
    pub fn publish_shown(&self, key: bool, timestamp_ms: u32, data: &[u8], shown: Option<Instant>) {
        let wire = encode_wire_frame(key, timestamp_ms, data);
        let mut state = self.lock_state();

        let Some(config) = state.config.clone() else {
            return;
        };

        let frame = Arc::new(Frame {
            key,
            wire,
            config,
            shown,
        });

        record_in_gop(&mut state.gop, &frame);

        let _ = self.frames.send(frame);
    }
}
