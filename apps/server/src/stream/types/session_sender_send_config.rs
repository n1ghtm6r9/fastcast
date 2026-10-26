use crate::hub::StreamConfig;
use crate::stream::SessionSender;
use wire::{FrameKind, OutgoingFrame};

impl SessionSender {
    pub fn send_config(&mut self, config: &StreamConfig) {
        let outgoing = OutgoingFrame {
            kind: FrameKind::Config,
            keyframe: false,
            payload: config.json.as_bytes(),
        };

        self.send_frame(&outgoing, config.fps);
    }
}
