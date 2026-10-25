use crate::packet::FrameKind;
use crate::receiver::{FrameReceiver, PendingFrame, ReceivedFrame};

impl FrameReceiver {
    pub(crate) fn release(&mut self, frame: PendingFrame) -> Option<ReceivedFrame> {
        match (frame.kind, frame.keyframe) {
            (FrameKind::Video, false) if self.waiting_for_keyframe => {
                self.video_lost_while_waiting = true;

                return None;
            }
            (FrameKind::Video, true) => {
                self.waiting_for_keyframe = false;
                self.video_lost_while_waiting = false;
            }
            _ => {}
        }

        Some(ReceivedFrame {
            number: frame.number,
            kind: frame.kind,
            keyframe: frame.keyframe,
            payload: frame.assemble(),
        })
    }
}
