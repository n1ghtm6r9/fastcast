use crate::packet::FrameKind;
use crate::packetizer::OutgoingFrame;

pub fn video_frame(payload: &[u8], keyframe: bool) -> OutgoingFrame<'_> {
    OutgoingFrame {
        kind: FrameKind::Video,
        keyframe,
        payload,
    }
}
