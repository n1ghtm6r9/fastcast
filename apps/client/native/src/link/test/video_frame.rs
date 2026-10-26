use wire::{FrameKind, ReceivedFrame};

pub(crate) fn video_frame(keyframe: bool, payload: &[u8]) -> ReceivedFrame {
    ReceivedFrame {
        number: 0,
        kind: FrameKind::Video,
        keyframe,
        payload: payload.to_vec(),
    }
}
