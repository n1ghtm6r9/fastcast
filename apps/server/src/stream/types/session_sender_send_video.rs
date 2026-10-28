use crate::hub::Frame;
use crate::stream::SessionSender;
use wire::{FrameKind, OutgoingFrame};

impl SessionSender {
    pub fn send_video(&mut self, frame: &Frame) {
        let outgoing = OutgoingFrame {
            kind: FrameKind::Video,
            keyframe: frame.key,
            payload: &frame.wire,
        };

        let number = self.send_frame(&outgoing, frame.config.fps);

        if let Some(shown) = frame.shown {
            self.control.sent(shown.elapsed());
        }

        if frame.key {
            self.last_key_frame = Some(number);
        }
    }
}
